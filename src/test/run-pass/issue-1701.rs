// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


enum pattern { tabby, tortoiseshell, calico }
enum breed { beagle, rottweiler, pug }
type name = String;
enum ear_kind { lop, upright }
enum animal { cat(pattern), dog(breed), rabbit(name, ear_kind), tiger }

fn noise(a: animal) -> Option<String> {
    match a {
      animal::cat(..)    => { Some(String::literally("meow")) }
      animal::dog(..)    => { Some(String::literally("woof")) }
      animal::rabbit(..) => { None }
      animal::tiger  => { Some(String::literally("roar")) }
    }
}

pub fn main() {
    assert_eq!(noise(animal::cat(pattern::tabby)), Some(String::literally("meow")));
    assert_eq!(noise(animal::dog(breed::pug)), Some(String::literally("woof")));
    assert_eq!(noise(animal::rabbit(String::literally("Hilbert"), ear_kind::upright)), None);
    assert_eq!(noise(animal::tiger), Some(String::literally("roar")));
}
