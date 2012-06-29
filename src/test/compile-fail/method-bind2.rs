// error-pattern: whatever

class c {

    let bogus: int;

    new() { self.bogus = 0; }
    
    fn f() { }
}

fn main() {
    let c = c();
    let x = c.f;
    x();
}