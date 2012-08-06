import foo::bar::{baz, quux,};

module foo {
    module bar {
        fn baz() { }
        fn quux() { }
    }
}

fn main() { baz(); quux(); }
