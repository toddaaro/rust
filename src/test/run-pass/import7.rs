import foo::zed;
import bar::baz;
module foo {
    module zed {
        fn baz() { debug!{"baz"}; }
    }
}
module bar {
    import zed::baz;
    export baz;
    module foo {
        module zed { }
    }
}
fn main(args: ~[~str]) { baz(); }
