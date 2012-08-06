module foo {

    export bar;

    module bar {
        fn y() { x(); }
    }

    fn x() { debug!{"x"}; }
}

fn main() { foo::bar::y(); }
