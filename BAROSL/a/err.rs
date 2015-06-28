#![feature(slice_patterns)]

fn main() {
    let buf = [b'a'; 4];
    match &buf {
        &[111, 0, 0, 0] => (),
        &[222, 0, 0, 0] => (),
        //b"aaaa" => println!("aaaa!!!"),
        //b"fals" => (),
        //_ => ()
    }
}
