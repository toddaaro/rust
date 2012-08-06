#[link(name="foreign_lib", vers="0.0")];

extern module rustrt {
    fn last_os_error() -> ~str;
}