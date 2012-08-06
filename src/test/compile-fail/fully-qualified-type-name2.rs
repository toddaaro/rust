// Test that we use fully-qualified type names in error messages.

module x {
    enum foo { }
}

module y {
    enum foo { }
}

fn bar(x: x::foo) -> y::foo {
    return x;
    //~^ ERROR mismatched types: expected `y::foo` but found `x::foo`
}

fn main() {
}
