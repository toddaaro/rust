// xfail-test
module a {
    type rust_task = uint;
    extern module rustrt {
        fn rust_task_is_unwinding(rt: *rust_task) -> bool;
    }
}

module b {
    type rust_task = bool;
    extern module rustrt {
        fn rust_task_is_unwinding(rt: *rust_task) -> bool;
    }
}

fn main() { }
