#[doc = "Reflection"];

export type_desc;
export get_type_desc;

#[doc = "A type descriptor"]
enum type_desc = {
    first_param: **libc::c_int,
    size: libc::size_t,
    align: libc::size_t
    // Remaining fields not listed
};

#[abi = "rust-intrinsic"]
native mod rusti {
    fn get_tydesc<T>() -> *();
}

#[doc = "
Returns a pointer to a type descriptor.

Useful for calling certain function in the Rust runtime or otherwise
performing dark magick.
"]
fn get_type_desc<T>() -> *type_desc {
    rusti::get_tydesc::<T>() as *type_desc
}
