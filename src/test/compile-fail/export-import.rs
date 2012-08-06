// error-pattern: import

import m::unexported;

module m {
    export exported;

    fn exported() { }

    fn unexported() { }
}


fn main() { unexported(); }
