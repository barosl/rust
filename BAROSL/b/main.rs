extern crate lib;
use lib::Foo;

/*
pub trait Foo {
    type Input=usize;
    fn bar(&self, _: Self::Input) {}
}

impl Foo for () {}
*/

fn main() {
    ().bar(5);
}
