enum a_tag<A,B> {
    varA(A),
    varB(B)
}

type t_rec<A,B> = {
    chA: u8,
    tA: a_tag<A,B>,
    chB: u8,
    tB: a_tag<A,B>
};

fn mk_rec<A:copy,B:copy>(a: A, b: B) -> t_rec<A,B> {
    ret { chA:0u8, tA:varA(a), chB:1u8, tB:varB(b) };
}

fn is_aligned<A>(amnt: uint, &&u: A) -> bool {
    let p = ptr::addr_of(u) as uint;
    ret (p & (amnt-1u)) == 0u;
}

fn variant_data_is_aligned<A,B>(amnt: uint, &&u: a_tag<A,B>) -> bool {
    alt u {
      varA(a) { is_aligned(amnt, a) }
      varB(b) { is_aligned(amnt, b) }
    }
}

#[cfg(target_arch = "x86")]
mod m {
    const u32align: uint = 4u;
    const u64align: uint = 4u;
}

#[cfg(target_arch = "x86_64")]
mod m {
    const u32align: uint = 4u;
    const u64align: uint = 8u;
}

fn main() {
    let x = mk_rec(22u64, 23u64);
    assert is_aligned(m::u64align, x.tA);
    assert variant_data_is_aligned(m::u64align, x.tA);
    assert is_aligned(m::u64align, x.tB);
    assert variant_data_is_aligned(m::u64align, x.tB);

    let x = mk_rec(22u64, 23u32);
    assert is_aligned(m::u64align, x.tA);
    assert variant_data_is_aligned(m::u64align, x.tA);
    assert is_aligned(m::u64align, x.tB);
    assert variant_data_is_aligned(m::u32align, x.tB);

    let x = mk_rec(22u32, 23u64);
    assert is_aligned(m::u64align, x.tA);
    assert variant_data_is_aligned(m::u32align, x.tA);
    assert is_aligned(m::u64align, x.tB);
    assert variant_data_is_aligned(m::u64align, x.tB);

    let x = mk_rec(22u32, 23u32);
    assert is_aligned(m::u32align, x.tA);
    assert variant_data_is_aligned(m::u32align, x.tA);
    assert is_aligned(m::u32align, x.tB);
    assert variant_data_is_aligned(m::u32align, x.tB);

    let x = mk_rec(22f64, 23f64);
    assert is_aligned(m::u64align, x.tA);
    assert variant_data_is_aligned(m::u64align, x.tA);
    assert is_aligned(m::u64align, x.tB);
    assert variant_data_is_aligned(m::u64align, x.tB);
}