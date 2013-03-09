// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct rust_dbg_struct {
    a: uint,
    b: uint
}

extern {
    fn rust_dbg_pass_struct(++s: rust_dbg_struct);
    fn rust_dbg_return_struct() -> rust_dbg_struct;
}

fn main() {
    let s = rust_dbg_struct { a: 10, b: 20 };
    rust_dbg_pass_struct(s);
    let s = rust_dbg_return_struct();
    fail_unless!(s.a == 10);
    fail_unless!(s.b == 20);
}