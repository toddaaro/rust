// error-pattern: whatever

impl i for int {
    fn f() { }
}

fn main() {
    let x = (10).f;
    x();
}