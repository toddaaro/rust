#[abi = "rust-intrinsic"]
extern module rusti {
    fn frame_address(f: fn(*u8));
}

fn main() {
    do rusti::frame_address |addr| {
        assert addr.is_not_null();
    }
}
