/*! Encapsulates translation relevant to unwinding */

pub trait UnwindStrategy {
    fn invoke(bcx: block, llfn: ValueRef, llargs: ~[ValueRef]) -> block;
}
