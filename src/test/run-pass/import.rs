module foo {
    fn x(y: int) { log(debug, y); }
}

module bar {
    import foo::x;
    import z = foo::x;
    fn thing() { x(10); z(10); }
}

fn main() { bar::thing(); }
