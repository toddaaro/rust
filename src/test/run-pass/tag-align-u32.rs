
enum a_tag {
    a_tag(u32)
}

type t_rec = {
    c8: u8,
    t: a_tag
};

fn mk_rec() -> t_rec {
    ret { c8:0u8, t:a_tag(0u32) };
}

fn is_4_byte_aligned(&&u: a_tag) -> bool {
    let p = ptr::addr_of(u) as u32;
    ret (p & 1u32) == 0u32;
}

fn main() {
    let x = mk_rec();
    assert is_4_byte_aligned(x.t);
}
