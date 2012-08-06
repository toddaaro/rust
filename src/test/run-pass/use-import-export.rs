

module foo {
    fn x() -> int { return 1; }
}

module bar {
    fn y() -> int { return 1; }
}

fn main() { foo::x(); bar::y(); }
