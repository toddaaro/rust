// error-pattern: import

module a { import foo = b::foo; export foo; }
module b { import foo = a::foo; export foo; }

fn main(args: ~[str]) { debug!{"loop"}; }
