// error-pattern:expected

import baz = foo::*;

module foo {
    fn bar() {}
}

fn main() {
}