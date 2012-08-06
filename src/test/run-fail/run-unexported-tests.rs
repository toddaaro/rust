// error-pattern:runned an unexported test
// compile-flags:--test

use std;

module m {
    export exported;

    fn exported() { }

    #[test]
    fn unexported() { fail ~"runned an unexported test"; }
}
