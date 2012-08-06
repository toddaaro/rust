


// -*- rust -*-
module foomod {
    fn foo<T>() { }
}

fn main() { foomod::foo::<int>(); foomod::foo::<int>(); }
