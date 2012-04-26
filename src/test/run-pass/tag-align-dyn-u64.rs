enum a_tag<A> {
    a_tag(A)
}

type t_rec = {
    c8: u8,
    t: a_tag<u64>
};

fn mk_rec() -> t_rec {
    ret { c8:0u8, t:a_tag(0u64) };
}

#[cfg(target_arch = "x86")]
fn mask() -> uint { 3u }

#[cfg(target_arch = "x86_64")]
fn mask() -> uint { 7u }

fn is_byte_aligned(&&u: a_tag<u64>) -> bool {
    let p = ptr::addr_of(u) as uint;
    ret (p & mask()) == 0u;
}

fn main() {
    let x = mk_rec();
    assert is_byte_aligned(x.t);
}
