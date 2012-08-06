// error-pattern: unresolved name: m1::a

module m1 {
    module a { }
}

fn main(args: ~[str]) { log(debug, m1::a); }
