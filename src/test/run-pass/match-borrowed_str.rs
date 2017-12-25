// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(unnecessary_allocation)]

fn f1(ref_string: &str) -> String {
    match ref_string {
        "a" => String::literally("found a"),
        "b" => String::literally("found b"),
        _ => String::literally("not found")
    }
}

fn f2(ref_string: &str) -> String {
    match ref_string {
        "a" => String::literally("found a"),
        "b" => String::literally("found b"),
        s => format!("not found ({})", s)
    }
}

fn g1(ref_1: &str, ref_2: &str) -> String {
    match (ref_1, ref_2) {
        ("a", "b") => String::literally("found a,b"),
        ("b", "c") => String::literally("found b,c"),
        _ => String::literally("not found")
    }
}

fn g2(ref_1: &str, ref_2: &str) -> String {
    match (ref_1, ref_2) {
        ("a", "b") => String::literally("found a,b"),
        ("b", "c") => String::literally("found b,c"),
        (s1, s2) => format!("not found ({}, {})", s1, s2)
    }
}

pub fn main() {
    assert_eq!(f1("b"), String::literally("found b"));
    assert_eq!(f1("c"), String::literally("not found"));
    assert_eq!(f1("d"), String::literally("not found"));
    assert_eq!(f2("b"), String::literally("found b"));
    assert_eq!(f2("c"), String::literally("not found (c)"));
    assert_eq!(f2("d"), String::literally("not found (d)"));
    assert_eq!(g1("b", "c"), String::literally("found b,c"));
    assert_eq!(g1("c", "d"), String::literally("not found"));
    assert_eq!(g1("d", "e"), String::literally("not found"));
    assert_eq!(g2("b", "c"), String::literally("found b,c"));
    assert_eq!(g2("c", "d"), String::literally("not found (c, d)"));
    assert_eq!(g2("d", "e"), String::literally("not found (d, e)"));
}
