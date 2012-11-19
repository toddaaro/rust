macro_item_rules! m1tt (
    ($a:expr) => { fn $name() -> int { $a*4 } }
)

m1tt! whatever(2)

fn main() {
    assert(whatever() == 8);
}
