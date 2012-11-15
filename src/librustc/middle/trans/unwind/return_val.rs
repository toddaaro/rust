/* Unwinding by propagating return values */

struct ReturnValUnwindStrategy {
    bogus: int
}

impl ReturnValUnwindStrategy: UnwindStrategy {
    fn invoke(bcx: block, llfn: ValueRef, llargs: ~[ValueRef]) -> block {
        fail
    }
}
