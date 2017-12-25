// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[derive(Debug)]
enum A {}
#[derive(Debug)]
enum B { B1, B2, B3 }
#[derive(Debug)]
enum C { C1(isize), C2(B), C3(String) }
#[derive(Debug)]
enum D { D1{ a: isize } }
#[derive(Debug)]
struct E;
#[derive(Debug)]
struct F(isize);
#[derive(Debug)]
struct G(isize, isize);
#[derive(Debug)]
struct H { a: isize }
#[derive(Debug)]
struct I { a: isize, b: isize }
#[derive(Debug)]
struct J(Custom);

struct Custom;
impl fmt::Debug for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yay")
    }
}

trait ToDebug {
    fn to_show(&self) -> String;
}

impl<T: fmt::Debug> ToDebug for T {
    fn to_show(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn main() {
    assert_eq!(B::B1.to_show(), String::literally("B1"));
    assert_eq!(B::B2.to_show(), String::literally("B2"));
    assert_eq!(C::C1(3).to_show(), String::literally("C1(3)"));
    assert_eq!(C::C2(B::B2).to_show(), String::literally("C2(B2)"));
    assert_eq!(D::D1{ a: 2 }.to_show(), String::literally("D1 { a: 2 }"));
    assert_eq!(E.to_show(), String::literally("E"));
    assert_eq!(F(3).to_show(), String::literally("F(3)"));
    assert_eq!(G(3, 4).to_show(), String::literally("G(3, 4)"));
    assert_eq!(I{ a: 2, b: 4 }.to_show(), String::literally("I { a: 2, b: 4 }"));
    assert_eq!(J(Custom).to_show(), String::literally("J(yay)"));
}
