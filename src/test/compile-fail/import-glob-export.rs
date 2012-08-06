
// error-pattern:unresolved name

import m1::*;

module m1 {
    export f1;
    fn f1() { }
    fn f2() { }
}

fn main() { f2(); }
