#![feature(slice_patterns)]

/*
fn f<T>(a: Option<&[T]>, b: Result<&[T], ()>) {
    match (&a, &b) {
        (&Some([]), &Ok([])) => (),
    }
}
*/

fn main() {
    /*
    let a: &[i32] = &[1];

    match a {
        [] => (),
        _ => (),
    }
    */

    let foo: &[u8] = &[1, 2, 3];
    match foo {
        &[a, b, c] => (),
        _ => (),
    }
}
