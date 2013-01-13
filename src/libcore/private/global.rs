use cast::{transmute, reinterpret_cast};
use clone::Clone;
use kinds::Owned;
use libc::{c_void, uintptr_t};
use option::{Option, Some, None};
use ops::Drop;
use pipes;
use private::{Exclusive, exclusive};
use private::{SharedMutableState, shared_mutable_state};
use private::{get_shared_immutable_state};
use private::at_exit::at_exit;
use send_map::linear::LinearMap;
use sys::Closure;
use task::spawn;
use uint;

pub type GlobalDataKey<T: Owned> = &fn(v: T);

pub unsafe fn global_data_clone_create<T: Owned Clone>(key: GlobalDataKey<T>, create: &fn() -> ~T) -> T {
    global_data_clone_create_(key_ptr(key), create)
}

unsafe fn global_data_clone_create_<T: Owned Clone>(key: uint, create: &fn() -> ~T) -> T {
    let mut clone_value: Option<T> = None;
    do global_data_modify_(key) |value: Option<~T>| {
        match value {
            None => {
                let value = create();
                clone_value = Some(value.clone());
                Some(value)
            }
            Some(value) => {
                clone_value = Some(value.clone());
                Some(value)
            }
        }
    }
    return clone_value.unwrap();
}

pub unsafe fn global_data_clone<T: Owned Clone>(key: GlobalDataKey<T>) -> Option<T> {
    global_data_clone_(key_ptr(key))
}

unsafe fn global_data_clone_<T: Owned Clone>(key: uint) -> Option<T> {
    do get_global_state().with_imm |gs| {
        match gs.map.find_ref(&key) {
            Some(&(ptr, _)) => {
                let ptr: &~T = unsafe { transmute(&ptr) };
                Some(ptr.clone())
            }
            None => None
        }
    }
}

pub unsafe fn global_data_modify<T: Owned>(key: GlobalDataKey<T>, op: &fn(Option<~T>) -> Option<~T>) {
    global_data_modify_(key_ptr(key), op)
}

unsafe fn global_data_modify_<T: Owned>(key: uint, op: &fn(Option<~T>) -> Option<~T>) {
    let mut old_dtor = None;
    do get_global_state().with |gs| unsafe {
        let (maybe_new_value, maybe_dtor) = match gs.map.pop(&key) {
            Some((ptr, dtor)) => {
                let value: ~T = transmute(ptr);
                (op(Some(value)), Some(dtor))
            }
            None => {
                (op(None), None)
            }
        };
        match maybe_new_value {
            Some(value) => {
                let data: *c_void = transmute(value);
                let dtor: ~fn() = match maybe_dtor {
                    Some(dtor) => dtor,
                    None => {
                        let dtor: ~fn() = || unsafe {
                            let _destroy_value: ~T = transmute(data);
                        };
                        dtor
                    }
                };
                let value = (data, dtor);
                gs.map.insert(key, value);
            }
            None => {
                match maybe_dtor {
                    Some(dtor) => old_dtor = Some(dtor),
                    None => ()
                }
            }
        }
    }
}

pub unsafe fn global_data_set<T: Owned>(key: GlobalDataKey<T>, value: ~T) {
    global_data_set_(key_ptr(key), value)
}

unsafe fn global_data_set_<T: Owned>(key: uint, value: ~T) {
    let data: *c_void = transmute(value);
    let mut old_value = None;
    do get_global_state().with |gs| unsafe {
        old_value = gs.map.pop(&key);
        let dtor: ~fn() = || unsafe {
            let _destroy_value: ~T = transmute(data);
        };
        let value = (data, dtor);
        gs.map.insert(key, value);
    }
    // Run the type dtor outside of the lock
    match old_value {
        Some((_, ref dtor)) => (*dtor)(),
        None => ()
    }
}

pub unsafe fn global_data_pop<T: Owned>(key: GlobalDataKey<T>) -> Option<~T> {
    global_data_pop_(key_ptr(key))
}

unsafe fn global_data_pop_<T: Owned>(key: uint) -> Option<~T> {
    do get_global_state().with |gs| {
        do gs.map.pop(&key).map |&(ptr, _)| {
            transmute(ptr)
        }
    }
}

// GlobalState is a map from keys to unique pointers and a
// destructor. Keys are pointers derived from the type of the
// global value.  There is a single GlobalState instance per runtime.
struct GlobalState {
    map: LinearMap<uint, (*c_void, ~fn())>
}

impl GlobalState: Drop {
    fn finalize(&self) {
        for self.map.each_value |v| {
            match v {
                &(_, ref dtor) => (*dtor)()
            }
        }
    }
}

fn get_global_state() -> Exclusive<GlobalState> unsafe {

    const POISON: int = -1;

    // XXX: Doing atomic_cxchg to initialize the global state
    // lazily, which wouldn't be necessary with a runtime written
    // in Rust
    let global_ptr = rust_get_global_data_ptr();

    if *global_ptr == 0 {
        // Global state doesn't exist yet, probably

        // The global state object
        let state = GlobalState {
            map: LinearMap()
        };

        // It's under a reference-counted mutex
        let state = ~exclusive(state);

        // Convert it to an integer
        let state_ptr: &Exclusive<GlobalState> = state;
        let state_i: int = transmute(state_ptr);

        // Swap our structure into the global pointer
        let prev_i = atomic_cxchg(&mut *global_ptr, 0, state_i);

        // Sanity check that we're not trying to reinitialize after shutdown
        assert prev_i != POISON;

        if prev_i == 0 {
            // Successfully installed the global pointer

            // Take a handle to return
            let clone = state.clone();

            // Install a runtime exit function to destroy the global object
            do at_exit || unsafe {
                // Poison the global pointer
                let prev_i = atomic_cxchg(&mut *global_ptr, state_i, POISON);
                assert prev_i == state_i;

                // Capture the global state object in the at_exit closure
                // so that it is destroyed at the right time
                let _capture_global_state = &state;
            };
            return clone;
        } else {
            // Somebody else initialized the globals first
            let state: &Exclusive<GlobalState> = transmute(prev_i);
            return state.clone();
        }
    } else {
        let state: &Exclusive<GlobalState> = transmute(*global_ptr);
        return state.clone();
    }
}

fn key_ptr<T: Owned>(key: GlobalDataKey<T>) -> uint unsafe {
    let closure: Closure = reinterpret_cast(&key);
    return transmute(closure.code);
}

extern {
    fn rust_get_global_data_ptr() -> *mut int;
}

#[abi = "rust-intrinsic"]
extern {
    fn atomic_cxchg(dst: &mut int, old: int, src: int) -> int;
}

#[test]
fn test_get_global_state() {
    let gs = get_global_state();
    do gs.with_imm |_| { }
}

#[test]
fn test_global_set() unsafe {
    fn key(_v: int) { }
    global_data_set(key, ~100);
}

#[test]
fn test_global_set_pop() unsafe {
    fn key(_v: int) { }
    global_data_set(key, ~100);
    assert global_data_pop(key) == Some(~100);
}

#[test]
fn test_global_set_pop_set() unsafe {
    fn key(_v: int) { }
    global_data_set(key, ~100);
    assert global_data_pop(key) == Some(~100);
    global_data_set(key, ~100);
}

#[test]
fn test_global_set_pop_multi_task() unsafe {
    fn key(_v: int) { }
    global_data_set(key, ~100);
    let (port, chan) = pipes::stream();
    do spawn unsafe {
        chan.send(global_data_pop(key));
    }

    assert port.recv() == Some(~100);

    global_data_set(key, ~200);
    let (port, chan) = pipes::stream();
    do spawn unsafe {
        chan.send(global_data_pop(key));
    }

    assert port.recv() == Some(~200);
}


#[test]
fn test_global_lazy_initialize_parallel() unsafe {
    fn key(_v: int) { }
    for uint::range(0, 100) |_| {
        do spawn unsafe {
            global_data_set(key, ~100);
        }
    }
}

#[test]
fn test_clone() unsafe {
    struct MyType {
        v: int
    }

    impl MyType: Clone {
        fn clone(&self) -> MyType {
            MyType {
                v: self.v
            }
        }
    }

    fn key(_v: MyType) { }

    global_data_set(key, ~MyType { v: 10 } );
    assert global_data_clone(key).get().v == 10;
}

#[test]
fn test_clone_rc() unsafe {
    type MyType = SharedMutableState<int>;

    fn key(_v: SharedMutableState<int>) { }

    for uint::range(0, 100) |_| {
        do spawn unsafe {
            let val = do global_data_clone_create(key) {
                ~shared_mutable_state(10)
            };

            assert get_shared_immutable_state(&val) == &10;
        }
    }
}

#[test]
fn test_modify() unsafe {
    type MyType = SharedMutableState<int>;

    fn key(_v: SharedMutableState<int>) { }

    do global_data_modify(key) |v| unsafe {
        match v {
            None => {
                Some(~shared_mutable_state(10))
            }
            _ => fail
        }
    }

    do global_data_modify(key) |v| {
        match v {
            Some(sms) => {
                let v = get_shared_immutable_state(sms);
                assert *v == 10;
                None
            },
            _ => fail
        }
    }

    do global_data_modify(key) |v| unsafe {
        match v {
            None => {
                Some(~shared_mutable_state(10))
            }
            _ => fail
        }
    }
}
