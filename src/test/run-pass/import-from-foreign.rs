module spam {
    fn ham() { }
    fn eggs() { }
}

#[abi = "cdecl"]
extern module rustrt {
    import spam::{ham, eggs};
    export ham;
    export eggs;
}

fn main() { rustrt::ham(); rustrt::eggs(); }
