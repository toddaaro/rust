
fn f(i: int) {
    let xxx = "test";
    let yyy = @0;
    if i != 0 {
        f(i - 1);
    } else {
        fail
    }
}

// -*- rust -*-
fn main() {
    f(100);
}
