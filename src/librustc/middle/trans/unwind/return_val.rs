/* Unwinding by propagating return values */

struct ReturnValUnwindStrategy {
    bogus: int
}

impl ReturnValUnwindStrategy: UnwindStrategy {
}
