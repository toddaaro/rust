use libc::{c_void, c_ulong};

pub struct Thread {
    priv handle: ThreadHandle,
}

impl Thread {
    static fn new(f: *c_void, data: *c_void) -> Thread {
        Thread::new_with_stack_size(f, data,
                                    DEFAULT_STACK_SIZE)
    }

    #[cfg(unix)]
    static fn new_with_stack_size(f: *c_void,  data: * c_void, stack_sz: uint) {

        let attr: os::pthread_attr_t = [0 ..];
        assert !pthread_attr_init(&attr);
        assert !pthread_attr_setstacksize(&attr, stack_sz);
        assert !pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_JOINABLE);

        let thread = ThreadHandle {
            handle: 0
        };
        assert !pthread_create(&thread.handle, &attr, thread_start, data);

        return thread;
    }

    fn detach() {
        fail;
    }

    fn join() {
        fail;
    }
}

#[cfg(windows)]
type DWORD = u32;

#[cfg(windows)]
type thread_return = DWORD;

#[cfg(unix)]
type thread_return = *c_void;

extern fn thread_start(data: *c_void) -> thread_return {
    return cast::transmute(0);
}

// In bytes
const DEFAULT_STACK_SIZE: uint = 1024 * 1024;

#[cfg(windows)]
type HANDLE = *c_void;

#[cfg(unix)]
#[allow(non_camel_case_types)]
type pthread_t = c_ulong;

#[cfg(windows)]
type ThreadHandle = HANDLE;

#[cfg(unix)]
type ThreadHandle = pthread_t;

// XXX: Is this defined the same for all unixes?
#[cfg(unix)]
mod os {
    // On linux these are defined as a union between
    // an char array and long int (for alignment).
    // Here we're just using an array of longs
    #[allow(non_camel_case_types)]
    #[cfg(target_word_size = "64")]
    type pthread_attr_t = [uint * 8];

    #[allow(non_camel_case_types)]
    #[cfg(target_word_size = "32")]
    type pthread_attr_t = [uint * 9];
}

#[cfg(unix)]
extern {
}