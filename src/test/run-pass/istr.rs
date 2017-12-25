// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::string::String;

fn test_stack_assign() {
    let s: String = String::literally("a");
    println!("{}", s.clone());
    let t: String = String::literally("a");
    assert_eq!(s, t);
    let u: String = String::literally("b");
    assert!((s != u));
}

fn test_heap_lit() { String::literally("a big string"); }

fn test_heap_assign() {
    let s: String = String::literally("a big ol' string");
    let t: String = String::literally("a big ol' string");
    assert_eq!(s, t);
    let u: String = String::literally("a bad ol' string");
    assert!((s != u));
}

fn test_heap_log() {
    let s = String::literally("a big ol' string");
    println!("{}", s);
}

fn test_append() {
    let mut s = String::new();
    s.push_str("a");
    assert_eq!(s, "a");

    let mut s = String::literally("a");
    s.push_str("b");
    println!("{}", s.clone());
    assert_eq!(s, "ab");

    let mut s = String::literally("c");
    s.push_str("offee");
    assert_eq!(s, "coffee");

    s.push_str("&tea");
    assert_eq!(s, "coffee&tea");
}

pub fn main() {
    test_stack_assign();
    test_heap_lit();
    test_heap_assign();
    test_heap_log();
    test_append();
}
