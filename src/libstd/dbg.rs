#[doc = "Unsafe debugging functions for inspecting values."];

#[abi = "cdecl"]
native mod rustrt {
    fn debug_tydesc(td: *refl::type_desc);
    fn debug_opaque<T>(td: *refl::type_desc, x: T);
    fn debug_box<T>(td: *refl::type_desc, x: @T);
    fn debug_tag<T>(td: *refl::type_desc, x: T);
    fn debug_fn<T>(td: *refl::type_desc, x: T);
    fn debug_ptrcast<T, U>(td: *refl::type_desc, x: @T) -> @U;
}

fn debug_tydesc<T>() {
    rustrt::debug_tydesc(refl::get_type_desc::<T>());
}

fn debug_opaque<T>(x: T) {
    rustrt::debug_opaque::<T>(refl::get_type_desc::<T>(), x);
}

fn debug_box<T>(x: @T) {
    rustrt::debug_box::<T>(refl::get_type_desc::<T>(), x);
}

fn debug_tag<T>(x: T) {
    rustrt::debug_tag::<T>(refl::get_type_desc::<T>(), x);
}

fn debug_fn<T>(x: T) {
    rustrt::debug_fn::<T>(refl::get_type_desc::<T>(), x);
}

unsafe fn ptr_cast<T, U>(x: @T) -> @U {
    ret rustrt::debug_ptrcast::<T, U>(refl::get_type_desc::<T>(), x);
}

fn refcount<T>(a: @T) -> uint unsafe {
    let p: *uint = unsafe::reinterpret_cast(a);
    ret *p;
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
