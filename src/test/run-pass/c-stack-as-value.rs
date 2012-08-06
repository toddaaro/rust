#[abi = "cdecl"]
extern module rustrt {
    fn get_task_id() -> libc::intptr_t;
}

fn main() {
    let _foo = rustrt::get_task_id;
}
