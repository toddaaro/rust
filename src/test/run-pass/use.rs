#[no_core];
use core;
use zed(name = "core");
use bar(name = "core", vers = "0.3");


import core::str;
import x = zed::str;
module baz {
    import bar::str;
    import x = core::str;
}

fn main() { }