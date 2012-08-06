module foo {
    export x;

    fn x() { bar::x(); }
}

module bar {
    export x;

    fn x() { debug!{"x"}; }
}

fn main() { foo::x(); }
