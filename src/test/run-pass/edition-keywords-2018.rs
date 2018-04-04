// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Zedition=2018 -Zunstable-options
// aux-build:edition-kw-macro-2015.rs

#![feature(raw_identifiers)]

#[macro_use]
extern crate edition_kw_macro_2015;

pub fn main() {
    let mut r#async = 1;

    r#async = consumes_async!(r#async);
    r#async = consumes_async_raw!(r#async);

    if consumes_ident!(r#async) == 1 {
        // ...
    }
    one::r#async();
    two::r#async();
}


mod one {
    produces_async! {}
}
mod two {
    produces_async_raw! {}
}
