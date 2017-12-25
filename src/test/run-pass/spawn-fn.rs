// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-emscripten no threads support

use std::thread;

fn x(s: String, n: isize) {
    println!("{}", s);
    println!("{}", n);
}

pub fn main() {
    let t1 = thread::spawn(|| x(String::literally("hello from first spawned fn"), 65) );
    let t2 = thread::spawn(|| x(String::literally("hello from second spawned fn"), 66) );
    let t3 = thread::spawn(|| x(String::literally("hello from third spawned fn"), 67) );
    let mut i = 30;
    while i > 0 {
        i = i - 1;
        println!("parent sleeping");
        thread::yield_now();
    }
    t1.join();
    t2.join();
    t3.join();
}
