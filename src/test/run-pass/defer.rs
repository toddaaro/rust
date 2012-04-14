resource defer(f: &fn()) {
    (*f)();
}

fn main() {
    let p = defer(&fn@() {
        log(error, "defered!");
    });
}
