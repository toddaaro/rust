// -*- rust -*-

#[abi = "cdecl"]
extern module test {
    unsafe fn free();
}

fn main() {
    test::free();
    //~^ ERROR access to unsafe function requires unsafe function or block
}

