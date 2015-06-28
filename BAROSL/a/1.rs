#![feature(slice_patterns)]

fn main() {
    let buf = &[b'c'; 4];
    //let buf = "cccc".as_bytes();
    match buf {
        b"aaaa" => println!("aaaa!!!"),
        &[222, 0, 0, 0] => (),
        b"bbbb" => println!("bbbb!!!"),
        //&[111, 0, 0, 0] => (),
        //b"fals" => (),
        _ => ()
    }
}

/*
fn main() {
    let buf = (b'a', b'a', b'a', b'a');
    match &buf {
        &(111, 0, 0, 0) => (),
        &(222, 0, 0, 0) => (),
    }
}
*/
