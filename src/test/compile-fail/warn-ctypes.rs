// compile-flags:-D ctypes
// error-pattern:found rust type
#[nolink]
extern module libc {
    fn malloc(size: int) -> *u8;
}

fn main() {
}