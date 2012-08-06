// error-pattern: unresolved name

module foo {
    export x;

    fn x() { bar::x(); }
}

module bar {
    export y;

    fn x() { debug!{"x"}; }

    fn y() { }
}

fn main() { foo::x(); }
