// error-pattern:unresolved
// xfail-test
import spam::{ham, eggs};

module spam {
    fn ham() { }
}

fn main() { ham(); eggs(); }
