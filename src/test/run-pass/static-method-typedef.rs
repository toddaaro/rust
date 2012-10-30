mod foo {
    pub struct FooShaper {
        dummy: int
    }

    pub impl FooShaper {
        static pub fn new() -> FooShaper {
            FooShaper { dummy: 1337 }
        }
    }
}

fn main() {
    type Shaper = foo::FooShaper;

    let sh = Shaper::new();
    assert 1 + sh.dummy == 1338;
}