
import baz::zed;
import zed::bar;

module baz {
    module zed {
        fn bar() { debug!{"bar2"}; }
    }
}

fn main() { bar(); }
