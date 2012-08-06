// error-pattern:expected

import baz = foo::{bar};

module foo {
    fn bar() {}
}

fn main() {
}