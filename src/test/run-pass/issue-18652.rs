// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Tests multiple free variables being passed by value into an unboxed
// once closure as an optimization by trans.  This used to hit an
// incorrect assert.

#![feature(unboxed_closures, overloaded_calls)]

fn main() {
    let x = 2u8;
    let y = 3u8;
    assert_eq!((move |:| x + y)(), 5);
}
