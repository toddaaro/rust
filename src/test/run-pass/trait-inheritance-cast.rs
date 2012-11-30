// Testing that supertrait methods can be called on subtrait object types

trait Foo {
    fn f() -> int;
}

trait Bar : Foo {
    fn g() -> int;
}

struct A {
    x: int
}

impl A : Foo {
    fn f() -> int { 10 }
}

impl A : Bar {
    fn g() -> int { 20 }
}

fn main() {
    let a = &A { x: 3 };
    let abar = a as &Bar;
    assert abar.f() == 10;
}

