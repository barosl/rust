pub trait Foo {
    type Input=usize;
    fn bar(&self, _: Self::Input) -> i64 {123}
}

impl Foo for () {}

fn main() {
    ().bar(5);
}
