// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use option::*;
use result::*;

// XXX: ~object doesn't work currently so these are some placeholder
// types to use instead
pub type EventLoopObject = super::uvio::UvEventLoop;
pub type IoFactoryObject = super::uvio::UvIoFactory;
pub type StreamObject = super::uvio::UvStream;
pub type TcpListenerObject = super::uvio::UvTcpListener;

pub trait EventLoop {
    fn run(&mut self);
    fn callback(&mut self, ~fn());
    /// The asynchronous I/O services. Not all event loops may provide one
    fn io(&mut self) -> Option<&'self mut IoFactoryObject>;
}

pub trait IoFactory {
    fn connect(&mut self, addr: IpAddr) -> Option<~StreamObject>;
    fn bind(&mut self, addr: IpAddr) -> Option<~TcpListenerObject>;
}

pub trait TcpListener {
    fn listen(&mut self) -> Option<~StreamObject>;
}

pub trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> Result<uint, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<(), ()>;
}

pub enum IpAddr {
    Ipv4(u8, u8, u8, u8, u16),
    Ipv6
}

/// A simple default implementation of EventLoop, without I/O
pub struct BasicEventLoop {
    queue: ~[~fn()]
}

impl BasicEventLoop {
    pub fn new() -> BasicEventLoop {
        BasicEventLoop {
            queue: ~[]
        }
    }

    pub fn run(&mut self) {
        while !self.queue.is_empty() {
            let next_cb = self.queue.shift();
            (next_cb)();
        }
    }
    pub fn callback(&mut self, cb: ~fn()) {
        self.queue.push(cb);
    }
    pub fn io(&mut self) -> Option<&'self mut IoFactoryObject> { None }
}

#[cfg(test)]
mod test {
    use super::BasicEventLoop;

    #[test]
    fn basic_event_loop_smoke_test() {
        let mut count = 0;
        let count_ptr: *mut int = &mut count;
        let mut event_loop = BasicEventLoop::new();

        do event_loop.callback {
            unsafe { *count_ptr +=1 };
        }

        do event_loop.callback {
            unsafe { *count_ptr +=1 };
        }

        do event_loop.callback {
            unsafe { *count_ptr +=1 };
        }

        event_loop.run();

        assert!(count == 3);
    }
}
