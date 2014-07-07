// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Maintains a shared list of sleeping schedulers. Schedulers
//! use this to wake each other up.

use std::sync::RWLock;
use std::sync::Arc;
use std::mem;

use sched::SchedHandle;


pub struct SleeperList {
    list: Arc<RWLock<Vec<SchedHandle>>>,
}

impl SleeperList {
    pub fn new(n: uint) -> SleeperList { 
        SleeperList{ list: Arc::new(RWLock::new(Vec::with_capacity(n))) }
    }
    pub fn add_sched(&self, sched: SchedHandle) {
        let mut raw = self.list.write();
        raw.push(sched);
        raw.downgrade();
    }
    pub fn remove_sched(&self, sched_id: uint) {
        let index = self.sched_index(sched_id);
        let mut raw = (*self.list).write();
        index.map(|i| { raw.swap_remove(i) });
        raw.downgrade();
    }
    pub fn get_neighbors(&self, sched_id: uint) -> (Option<&mut SchedHandle>, Option<&mut SchedHandle>) {
        let index = self.sched_index(sched_id);
        let raw = self.list.read();
        match index {
            None => { (None, None) }
            Some(i) => {
                let last = raw.len() - 1;
                let right = if last > i { 
                    Some(raw.get(i + 1))
                } else { 
                    None 
                };
                let left = if i > 0 { 
                    Some(raw.get(i - 1))
                } else if last > i + 1 { 
                    Some(raw.get(last))
                } else { 
                    None 
                };
                unsafe { mem::transmute((left, right)) }
            }
        }
    }            
    pub fn sched_index(&self, sched_id: uint) -> Option<uint> {
        let raw = self.list.read();
        for i in range(0u, raw.len()) {
            if raw.get(i).sched_id == sched_id {
                return Some(i);
            };
        };
        return None;
    }
}

impl Clone for SleeperList {
    fn clone(&self) -> SleeperList {
        SleeperList { list: self.list.clone() }   
    }
}    

