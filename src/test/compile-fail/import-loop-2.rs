// error-pattern:import

module a {
    import b::x;
    export x;
}

module b {
    import a::x;
    export x;

    fn main() { let y = x; }
}
