fn f() -> int {
    module m {
        fn g() -> int { 720 }
    }

    m::g()
}

fn main() {
    assert f() == 720;
}