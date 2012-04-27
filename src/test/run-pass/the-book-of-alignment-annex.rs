// Testing that shape code understands alignment of odd structs

fn main() {
    enum mup { breh, feh(u16, u32, u8), meh(float) }
    let val = #fmt("%?", feh(10u16, 20u32, 30u8));
    log(debug, val);
    assert val == "feh(10, 20, 30)";

    dup_tup_blup();
}

#[cfg(target_arch = "x86")]
fn dup_tup_blup() {
    enum dup { thats_righty }
    enum tup { }
    enum blup = int;

    assert sys::min_align_of::<dup>() == 4u;
    assert sys::min_align_of::<tup>() == 4u;
    assert sys::min_align_of::<blup>() == 4u;
}

#[cfg(target_arch = "x86_64")]
fn dup_tup_blup() {
    enum dup { thats_righty }
    enum tup { }
    enum blup = int;

    assert sys::min_align_of::<dup>() == 8u;
    assert sys::min_align_of::<tup>() == 8u;
    assert sys::min_align_of::<blup>() == 8u;
}
