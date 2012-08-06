// Export the enum variants, without the enum

module foo {
    export t1;
    enum t { t1, }
}

fn main() { let v = foo::t1; }
