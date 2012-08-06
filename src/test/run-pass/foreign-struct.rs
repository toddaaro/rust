// xfail-win32
// Passing enums by value

enum void { }

#[nolink]
extern module bindgen {
    fn printf(++v: void);
}

fn main() { }
