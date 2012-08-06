// compile-flags:-D ctypes

#[allow(ctypes)];

#[nolink]
extern module libc {
    fn malloc(size: int) -> *u8;
}

fn main() {
}