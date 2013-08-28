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
//use unstable::intrinsics;
use unstable::atomics::{AtomicPtr, SeqCst, AtomicUint, Relaxed, Acquire, Release, fence};
//use unstable::atomics::{AtomicPtr, SeqCst, AtomicUint, Relaxed, Acquire};
use cast::{transmute, forget};
//use rt::thread::*;
//use vec;
use prelude::*;
//use clone::Clone;
//use ops::Drop;
//use rt::global_heap;
use ptr;
//use borrow::to_uint;
//use unstable::raw::Vec;
use rt::global_heap::malloc_raw;
use sys;

pub struct WorkstealingDeque<T> {
    top: AtomicUint,
    bottom: AtomicUint,
    array: AtomicPtr<DequeArray<T>>
}

pub struct DequeArray<T> {
    size: AtomicUint,
    raw: *AtomicPtr<T>
}

impl<T: Send> DequeArray<T> {
    pub fn new(size: uint) -> DequeArray<T> {
        unsafe {
            let ptr = malloc_raw((1 << size) * sys::nonzero_size_of::<AtomicPtr<T>>());
            return DequeArray {
                size: AtomicUint::new(size),
                raw: transmute(ptr)
            };
        }
    }
    pub fn new_default() -> DequeArray<T> {
        DequeArray::new(20)
    }
    pub fn index<T>(&self, i: uint) -> *mut AtomicPtr<T> {
//        rterrln!("calling index on: %u", i);
        unsafe {
            let res = transmute(ptr::offset::<AtomicPtr<T>>(transmute(self.raw), i as int));
//            rterrln!("start: %u, res: %u", transmute(self.raw), transmute(res));
            return res;
        }
    }
    pub fn grow(&mut self) {
        unsafe {
            let old_size = self.size.load(SeqCst);
            let new_size = old_size + 1;
            
            let old_ptr = self.raw;

//            let new: *mut DequeArray<T> = transmute(&mut ~DequeArray::new::<T>(new_size));
//            let new_ptr = (*new).raw;

            let new_ptr = malloc_raw((1 << new_size) * sys::nonzero_size_of::<AtomicPtr<T>>());


//            ptr::copy_nonoverlapping_memory::<AtomicPtr<T>>(transmute(new_ptr), transmute(old_ptr), 1 << old_size);

            for i in range(0u, 1 << old_size) {
                let mut new_opt: AtomicPtr<T> = 
                    transmute(ptr::offset::<AtomicPtr<T>>(transmute(new_ptr), i as int));
                let old_opt: AtomicPtr<T> = 
                    transmute(ptr::offset::<AtomicPtr<T>>(transmute(old_ptr), i as int));
                new_opt.store(old_opt.load(SeqCst), SeqCst);
            };
            
            self.size.store(new_size, SeqCst);
            self.raw = transmute(new_ptr);

            forget(old_ptr);
//            return transmute(new);
        }
    }
}

impl<T: Send> WorkstealingDeque<T> {
    pub fn new() -> WorkstealingDeque<T> {
        unsafe {
            WorkstealingDeque {
                top: AtomicUint::new(0),
                bottom: AtomicUint::new(0),
                array: AtomicPtr::new(transmute(~DequeArray::new_default::<T>())) 
            }
        }
    }

    pub fn push(&mut self, value: ~T) {
        unsafe {
            let b = self.bottom.load(Relaxed);
            let t = self.top.load(Acquire);
            let a = self.array.load(Relaxed);
            let size = 1 << (*a).size.load(SeqCst); // XXX: Pick ordering.
            if b - t > size - 1 {
//                rterrln!("growing: %?, size: %u", value, size);
//                let at: &mut DequeArray<T> = transmute(a);
//                at.grow();
                rtabort!("growth disabled");
            }
//            let a: *mut DequeArray<T> = transmute(self.array.load(Relaxed));
            let size = 1 << (*a).size.load(SeqCst); // XXX: Pick ordering.
            let value: *mut T = transmute(value);
            (*(*a).index::<T>(b % size)).store(value, Relaxed);
            fence(Release);
            self.bottom.store(b+1, Relaxed);                        
//            rterrln!("pushed value");
        }
    }

    pub fn pop(&mut self) -> Option<~T> {
        unsafe {
            let b = self.bottom.load(Relaxed) - 1;
            let a = self.array.load(Relaxed);
            self.bottom.store(b, Relaxed);
            fence(SeqCst);
            let t = self.top.load(Relaxed);
//            rtdebug!("in pop t = %u b = %u", t, b);
            let mut x: Option<~T>;
            if t <= b {
                let size = 1 << (*a).size.load(SeqCst); // XXX: Pick ordering.
                let v: ~T = transmute((*(*a).index::<T>(b % size)).load(Relaxed));
                x = Some(v);
                if t == b {
                    // XXX: "compare_exchange_strong_explicit" is what in rust?
                    if t != self.top.compare_and_swap(t, t+1, SeqCst) {
                        forget(x);
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
            let mut x: Option<~T>;
            if (t < b) {
                let a = self.array.load(Relaxed);
                let size = 1 << (*a).size.load(SeqCst); // XXX: Pick ordering.
                x = Some(transmute((*(*a).index::<T>(t % size)).load(Relaxed)));
                // XXX: "compare_exchange_strong_explicit" is what in rust?
                if t != self.top.compare_and_swap(t, t+1, SeqCst) {
                    forget(x);
//                    rterrln!("steal failed, found: %?", x);
                    return None;
                }
            } else {
                x = None;
            }
//            rterrln!("steal returning");
            return x;
        }
    }

    pub fn is_empty(&self) -> bool {
        // XXX: This is almost certainly too restrictive an ordering.
        let b = self.bottom.load(SeqCst);
        let t = self.top.load(SeqCst);
        return b == t;
    }

    pub fn size(&self) -> uint {
        let b = self.bottom.load(SeqCst);
        let t = self.top.load(SeqCst);
        return b-t;
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
    unsafe {
    rtdebug!("about to start test");

    let mut deque: WorkstealingDeque<uint> = WorkstealingDeque::new::<uint>();
    rtdebug!("about to push 1");
    let mut a = ~1;
    rterrln!("a: %u", transmute(&mut *a));
    deque.push(a);
    rtdebug!("about to push 2");
    let mut b = ~2;
    rterrln!("b: %u", transmute(&mut *b));
    deque.push(b);
    rtdebug!("about to pop one");
    let mut one = deque.pop().unwrap();
    rtdebug!("about to steal two");
    let mut two = deque.steal().unwrap();
//    rterrln!("first: %u", transmute(&mut *one));
//    rterrln!("second: %u", transmute(&mut *two));
    assert!(3 == *one + *two);
}
}

#[test]
pub fn test_chase_lev_multithreaded() {

    use prelude::*;
//    use rt::comm::*;
    use cell::Cell;
//    use unstable::run_in_bare_thread;
    use rt::thread::Thread;

    let mut deque: WorkstealingDeque<uint> = WorkstealingDeque::new();
    let deque_ptr: *mut WorkstealingDeque<uint> = &mut deque;

    // Number of stealers
    let m = 1;
    // Number of elements per stealer
    let n = 900000;

    // Destination array
    let mut dest = ~[0,0,0,0,0,0,0,0];

    let dest_ptr: *mut ~[uint] = &mut dest;

    // threads
    let mut threads = ~[];

    // Pipes to report doneness
//    let (port, chan) = stream::<uint>();
//    let shared_chan = SharedChan::new(chan);
/*
    do spawn {
        rterrln!("spawning inserter task");
        for i in range (1u, n*m+1) {            
            unsafe {
                (*deque_ptr).push(~i);
//                rterrln!("inserting: %u", i);
            }            
        }        
    }
*/
    for i in range(0u, m) {
//        rterrln!("spawning stealer");
        let dest = Cell::new(dest_ptr);
        let thread = do Thread::start {
//            rterrln!("%u stealer spawned", i);
            let mut res: uint = 0;
            let mut count: uint = 0;
            unsafe {
                while count < n {
                    match (*deque_ptr).steal() {                        
                        Some(~x) => {
//                            rterrln!("stole: %u, size: %u, count: %u", x, (*deque_ptr).size(), count);
                            res += x;
                            count += 1;
                        }
                        None => {
                            if i==0 {
//                               rterrln!("%u failed to steal at count = %u, size=%u", i, count, (*deque_ptr).size());
                            }
//                            rterrln!("failed to steal");
                            ()
                        }
                    }
                }
//                rterrln!("%u done stealing", i);
            let dest = dest.take();
//            rterrln!("setting dest ptr");
            (*dest)[i] = res;
//            rterrln!("dest ptr set");
            }
        };
        threads.push(thread);
    }

    let main_thread = do Thread::start {
//        rterrln!("spawning inserter task");
        for i in range (1u, n*m+1) {
            unsafe {
                (*deque_ptr).push(~i)
//                rterrln!("inserting: %u", i);
            }
        }
//        rterrln!("done inserting");
    };
    
    for thread in threads.move_iter() {
        thread.join();
    }
    main_thread.join();

    let mut res: uint = 0;
    for i in range(0u, m) {
        res += dest[i];
    }

    let expected_res: uint = (n*m) * (n*m + 1) / 2;
    rterrln!("found: %u, expected: %u", res, expected_res);

    assert!(res == expected_res);
    
}