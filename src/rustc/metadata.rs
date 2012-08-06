// Define the rustc API's that the metadata module has access to
// Over time we will reduce these dependencies and, once metadata has
// no dependencies on rustc it can move into its own crate.

module middle {
    import ty = middle_::ty;
    export ty;
}

module front {
}

module back {
}

module driver {
}

module util {
    import ppaux = util_::ppaux;
    export ppaux;
}

module lib {
    import llvm = lib_::llvm;
    export llvm;
}
