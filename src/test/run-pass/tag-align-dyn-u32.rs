enum a_tag<A> {
    a_tag(A)
}

type t_rec = {
    c8: u8,
    t: a_tag<u32>
};

fn mk_rec() -> t_rec {
    ret { c8:0u8, t:a_tag(0u32) };
}

fn is_4_byte_aligned(&&u: a_tag<u32>) -> bool {
    let p = ptr::addr_of(u) as uint;
    ret (p & 3u) == 0u;
}

fn main() {
    let x = mk_rec();
    #error("align %?", sys::min_align_of::<a_tag<u32>>());
    assert is_4_byte_aligned(x.t);
}
