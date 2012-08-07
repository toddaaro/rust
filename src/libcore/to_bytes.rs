trait ToBytes {
    fn to_bytes() -> ~[u8];
}

impl of ToBytes for ~[u8] {
    fn to_bytes() -> ~[u8] { copy self }
}

impl of ToBytes for @~[u8] {
    fn to_bytes() -> ~[u8] { copy *self }
}

impl of ToBytes for ~str {
    fn to_bytes() -> ~[u8] { str::bytes(self) }
}

impl of ToBytes for @(~str) {
    fn to_bytes() -> ~[u8] { str::bytes(*self) }
}
