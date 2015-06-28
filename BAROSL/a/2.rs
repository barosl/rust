#![feature(slice_patterns)]

fn main() {
    let a = &[1,2,3,4];
    match a {
        &[1,2,3,4] => (),
        &[4,5,6,7] => (),
    }
}

/*
enum A {
    A([i32; 3]),
}

fn main() {
    let a = A::A([1,2,3]);
    match a {
        A::A([1,2,3]) => (),
    }
}
*/
