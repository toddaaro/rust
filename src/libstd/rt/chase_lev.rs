// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Some reference stuff:
// http://en.cppreference.com/w/cpp/atomic/atomic_compare_exchange
// http://www.di.ens.fr/~zappa/readings/ppopp13.pdf <- code from here
// http://gcc.gnu.org/onlinedocs/gcc-4.3.5/gcc/Atomic-Builtins.html

use option::*;
use kinds::Send;
use unstable::intrinsics;
use unstable::atomics::{AtomicPtr, SeqCst, AtomicUint, Relaxed, Acquire, Release, fence};
//use unstable::atomics::{AtomicPtr, SeqCst, AtomicUint, Relaxed, Acquire};
//use cast::{transmute, forget};
//use clone::Clone;
//use ops::Drop;
use cast::transmute;

pub struct WorkstealingDeque<T> {
    count: AtomicUint,
    top: AtomicUint,
    bottom: AtomicUint,
    array: AtomicPtr<DequeArray<T>>
}

pub struct DequeArray<T> {
    size: AtomicUint,
    raw: ~[AtomicPtr<T>]
}

impl<T: Send> DequeArray<T> {
    pub fn new() -> DequeArray<T> {
        unsafe {
            DequeArray {
                size: AtomicUint::new(80000),
                raw: ~[AtomicPtr::new(intrinsics::uninit()), ..80000]
            }
        }
    }
}

impl<T: Send> WorkstealingDeque<T> {
    pub fn new() -> WorkstealingDeque<T> {
        unsafe {
            WorkstealingDeque {
                count: AtomicUint::new(1),
                top: AtomicUint::new(0),
                bottom: AtomicUint::new(0),
                array: AtomicPtr::new(transmute(~DequeArray::new::<T>())) 
            }
        }
    }

    pub fn push(&mut self, value: ~T) {
        unsafe {
            let b = self.bottom.load(Relaxed);
            let t = self.top.load(Acquire);
            let a: *mut DequeArray<T> = self.array.load(Relaxed);
            let size = (*a).size.load(SeqCst); // XXX: Pick ordering.
            if b - t > size - 1 {
                rtabort!("deque overfilled, no resize implemented");
                //self.resize();
            }
            let size = (*a).size.load(SeqCst); // XXX: Pick ordering.
            let value: *mut T = transmute(value);
            rtdebug!("about to push onto raw array: length is: %u", (*a).raw.len());
            (*a).raw[b % size].store(value, Relaxed);
            fence(Release);
            self.bottom.store(b+1, Relaxed);                        
        }
    }

    pub fn pop(&mut self) -> Option<~T> {
        unsafe {
            let b = self.bottom.load(Relaxed) - 1;
            let a = self.array.load(Relaxed);
            self.bottom.store(b, Relaxed);
            fence(SeqCst);
            let t = self.top.load(Relaxed);
            rtdebug!("in pop t = %u b = %u", t, b);
            let mut x: Option<~T>;
            if t <= b {
                let size = (*a).size.load(SeqCst); // XXX: Pick ordering.
                x = Some(transmute((*a).raw[b % size].load(Relaxed)));
                if t == b {
                    // XXX: "compare_exchange_strong_explicit" is what in rust?
                    if t != self.top.compare_and_swap(t, t+1, SeqCst) {
                        x = None;
                    }
                    self.bottom.store(b+1, Relaxed);
                }
            } else {
                x = None;
                self.bottom.store(b+1, Relaxed);
            }
            return x;
        }
    }

    pub fn steal(&mut self) -> Option<~T> {
        unsafe {
            let t = self.top.load(Acquire);
            fence(SeqCst);
            let b = self.bottom.load(Acquire);
            let mut x: Option<~T> = None;
            if (t < b) {
                let a = self.array.load(Relaxed);
                let size = (*a).size.load(SeqCst); // XXX: Pick ordering.
                x = Some(transmute((*a).raw[t % size].load(Relaxed)));
                // XXX: "compare_exchange_strong_explicit" is what in rust?
                if t != self.top.compare_and_swap(t, t+1, SeqCst) {
                    return None;
                }
            }
            return x;
        }
    }

    pub fn is_empty(&self) -> bool {
        // XXX: This is almost certainly too restrictive an ordering.
        let b = self.bottom.load(SeqCst);
        let t = self.top.load(SeqCst);
        return b == t;
    }
}

/*
impl<T> Clone for WorkstealingDeque<T> {
    fn clone(&self) -> WorkstealingDeque<T> {
        unsafe {            
            let mut new_copy: WorkstealingDeque<T> = transmute(self);
            let old_count = new_copy.count.fetch_add(1, SeqCst);
            assert!(old_count >= 1);
            return new_copy;
        }
    }
}
*/
         

#[test]
pub fn test_chase_lev_trivial() {

    use prelude::*;

    rtdebug!("about to start test");

    let mut deque: WorkstealingDeque<int> = WorkstealingDeque::new::<int>();
    rtdebug!("about to push 1");
    deque.push(~1);
    rtdebug!("about to push 2");
    deque.push(~2);
    rtdebug!("about to pop one");
    let one = deque.pop().unwrap();
    rtdebug!("about to steal two");
    let two = deque.steal().unwrap();
    assert!(3 == *one + *two);
}

#[test]
pub fn test_chase_lev_multithreaded() {

    use prelude::*;
    use rt::comm::*;
    use cell::Cell;

    let mut deque: WorkstealingDeque<uint> = WorkstealingDeque::new();
    let deque_ptr: *mut WorkstealingDeque<uint> = &mut deque;

    // Number of stealers
    let m = 8;
    // Number of elements per stealer
    let n = 1000000;

    // Pipes to report doneness
    let (port, chan) = stream::<uint>();
    let shared_chan = SharedChan::new(chan);

    do spawn {
//        rtdebug!("spawning inserter task");
        for i in range (1u, n*m+1) {            
            unsafe {
                (*deque_ptr).push(~i);
//                rterrln!("inserting: %u", i);
            }
        }        
    }
    for _ in range(0u, m) {
//        rterrln!("spawning stealer");
        let dest = Cell::new(shared_chan.clone());
        do spawn {
            let mut res: uint = 0;
            let mut count: uint = 0;
            unsafe {
                while count < n {
                    match (*deque_ptr).steal() {
                        Some(~x) => {
//                            rterrln!("stole: %u", x);
                            res += x;
                            count += 1;
                        }
                        None => {
//                            rterrln!("failed to steal");
                        }
                    }
                }
//                rterrln!("done stealing");
            }
            dest.take().send(res);
        }
    }
    
    let mut res: uint = 0;
    for _ in range(0u, m) {
        res += port.recv()
    }

    let expected_res: uint = (n*m) * (n*m + 1) / 2;
    rterrln!("found: %u, expected: %u", res, expected_res);

    assert!(res == expected_res);
    
}