// error-pattern: unresolved
import baz::zed::bar;
module baz { }
module zed {
    fn bar() { debug!{"bar3"}; }
}
fn main(args: ~[str]) { bar(); }
