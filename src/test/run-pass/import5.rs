import foo::bar;
module foo {
    import zed::bar;
    export bar;
    module zed {
        fn bar() { debug!{"foo"}; }
    }
}

fn main(args: ~[~str]) { bar(); }
