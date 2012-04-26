
enum a_tag {
    a_tag(u64)
}

type t_rec = {
    c8: u8,
    t: a_tag
};

fn mk_rec() -> t_rec {
    ret { c8:0u8, t:a_tag(0u64) };
}

// 4 byte alignment on x86 unix
#[cfg(target_arch = "x86")]
fn mask() -> u64 { 3u64 }

// 8 byte alignment on x86_64
#[cfg(target_arch = "x86_64")]
fn mask() -> u64 { 7u64 }

fn is_byte_aligned(&&u: a_tag) -> bool {
    let p = ptr::addr_of(u) as u64;
    ret (p & mask()) == 0u64;
}

fn main() {
    let x = mk_rec();
    assert is_byte_aligned(x.t);
}
