module a {
    module b {
        module a {
            fn foo() -> int { return 1; }
        }
    }
}

module c {
    import a::b::a;
    fn bar() { assert (a::foo() == 1); }
}

fn main() { c::bar(); }
