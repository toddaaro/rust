// error-pattern:import

import y::x;

module y {
    import x;
    export x;
}

fn main() { }
