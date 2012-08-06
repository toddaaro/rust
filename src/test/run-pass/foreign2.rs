#[abi = "cdecl"]
#[nolink]
extern module bar { }

#[abi = "cdecl"]
#[nolink]
extern module zed { }

#[abi = "cdecl"]
#[nolink]
extern module libc {
    fn write(fd: int, buf: *u8,
             count: core::libc::size_t) -> core::libc::ssize_t;
}

#[abi = "cdecl"]
#[nolink]
extern module baz { }

fn main(args: ~[~str]) { }
