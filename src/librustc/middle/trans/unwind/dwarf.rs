/*! DWARF-based unwinding using LLVM's exception infrastructure */

struct DwarfUnwindStrategy {
    bogus: int
}

impl DwarfUnwindStrategy: UnwindStrategy {
}
