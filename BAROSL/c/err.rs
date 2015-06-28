#![feature(no_std)]
#![no_std]
macro_rules! buh {
    (
        $(#[$attr:meta])*
        fn bad();
    ) => {
        $(#[$attr])*
        fn bad() {
        }
    }
}

buh! {
    /// `\`
    fn bad();
}
