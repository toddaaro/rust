
import foo::x;
import z = foo::x;

module foo {
    fn x(y: int) { log(debug, y); }
}

fn main() { x(10); z(10); }
