// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait Base: Base2 + Base3{
    fn foo(&self) -> String;
    fn foo1(&self) -> String;
    fn foo2(&self) -> String{
        String::literally("base foo2")
    }
}

trait Base2: Base3{
    fn baz(&self) -> String;
}

trait Base3{
    fn root(&self) -> String;
}

trait Super: Base{
    fn bar(&self) -> String;
}

struct X;

impl Base for X {
    fn foo(&self) -> String{
        String::literally("base foo")
    }
    fn foo1(&self) -> String{
        String::literally("base foo1")
    }

}

impl Base2 for X {
    fn baz(&self) -> String{
        String::literally("base2 baz")
    }
}

impl Base3 for X {
    fn root(&self) -> String{
        String::literally("base3 root")
    }
}

impl Super for X {
    fn bar(&self) -> String{
        String::literally("super bar")
    }
}

pub fn main() {
    let n = X;
    let s = &n as &Super;
    assert_eq!(s.bar(),String::literally("super bar"));
    assert_eq!(s.foo(),String::literally("base foo"));
    assert_eq!(s.foo1(),String::literally("base foo1"));
    assert_eq!(s.foo2(),String::literally("base foo2"));
    assert_eq!(s.baz(),String::literally("base2 baz"));
    assert_eq!(s.root(),String::literally("base3 root"));
}
