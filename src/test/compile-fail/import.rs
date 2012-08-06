// xfail-test
// error-pattern: unresolved
import zed::bar;
import zed::baz;
module zed {
    fn bar() { debug!{"bar"}; }
}
fn main(args: ~[str]) { bar(); }
