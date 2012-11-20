fn macros() {

    macro_item_rules! int_template (
        ($self:path, $bits:expr) => {
            mod $name {

                const bits: uint = $bits;
                
                pub pure fn min(x: $self, y: $self) -> $self { if x < y { x } else { y } }
            }
        }
    )
}

#[cfg(target_arch = "x86_64")]
int_template! intmod (int, 64)

#[cfg(target_arch = "x86")]
int_template! intmod (int, 64)

fn main() {
    assert intmod::min(0, 1) == 0;
}
