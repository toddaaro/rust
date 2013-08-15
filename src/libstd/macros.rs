// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_escape];

macro_rules! rterrln (
    ($( $arg:expr),+) => ( {
        ::rt::util::dumb_println(fmt!( $($arg),+ ));
    } )
)

// Some basic logging. Enabled by passing `--cfg rtdebug` to the libstd build.
macro_rules! rtdebug (
    ($( $arg:expr),+) => ( {
        if cfg!(rtdebug) {
            use rt::task::{Task, SchedTask, GreenTask};
            use rt::local::Local;
            if Local::exists::<Task>() {
                do Local::borrow::<Task,()> |task| {
                    let task_id = task.task_id;
                    let is_sched = match task.task_type {
                        SchedTask => true,
                        _ => false
                    };
                    let out = if is_sched {
                        fmt!("[%u][S] ", task_id)
                    } else {
                        fmt!("[%u] ", task_id)
                    };
                    let raw_out = out + fmt!( $($arg),+ );
                    ::rt::util::dumb_println(raw_out);                    
                }
            } else {
                ::rt::util::dumb_println("[?] " + fmt!( $($arg),+ ));
            }
        }
    })
)

macro_rules! rtassert (
    ( $arg:expr ) => ( {
        if !$arg {
            rtabort!("assertion failed: %s", stringify!($arg));
        }
    } )
)


macro_rules! rtabort(
    ($( $msg:expr),+) => ( {
        ::rt::util::abort(fmt!($($msg),+));
    } )
)

