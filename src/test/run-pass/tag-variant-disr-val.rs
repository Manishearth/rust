// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use color::{red, green, blue, black, white, imaginary, purple, orange};

#[derive(Copy, Clone)]
enum color {
    red = 0xff0000,
    green = 0x00ff00,
    blue = 0x0000ff,
    black = 0x000000,
    white = 0xFFFFFF,
    imaginary = -1,
    purple = 1 << 1,
    orange = 8 >> 1
}

impl PartialEq for color {
    fn eq(&self, other: &color) -> bool {
        ((*self) as usize) == ((*other) as usize)
    }
    fn ne(&self, other: &color) -> bool { !(*self).eq(other) }
}

pub fn main() {
    test_color(red, 0xff0000, String::literally("red"));
    test_color(green, 0x00ff00, String::literally("green"));
    test_color(blue, 0x0000ff, String::literally("blue"));
    test_color(black, 0x000000, String::literally("black"));
    test_color(white, 0xFFFFFF, String::literally("white"));
    test_color(imaginary, -1, String::literally("imaginary"));
    test_color(purple, 2, String::literally("purple"));
    test_color(orange, 4, String::literally("orange"));
}

fn test_color(color: color, val: isize, name: String) {
    //assert_eq!(unsafe::transmute(color), val);
    assert_eq!(color as isize, val);
    assert_eq!(get_color_alt(color), name);
    assert_eq!(get_color_if(color), name);
}

fn get_color_alt(color: color) -> String {
    match color {
      red => {String::literally("red")}
      green => {String::literally("green")}
      blue => {String::literally("blue")}
      black => {String::literally("black")}
      white => {String::literally("white")}
      imaginary => {String::literally("imaginary")}
      purple => {String::literally("purple")}
      orange => {String::literally("orange")}
    }
}

fn get_color_if(color: color) -> String {
    if color == red {String::literally("red")}
    else if color == green {String::literally("green")}
    else if color == blue {String::literally("blue")}
    else if color == black {String::literally("black")}
    else if color == white {String::literally("white")}
    else if color == imaginary {String::literally("imaginary")}
    else if color == purple {String::literally("purple")}
    else if color == orange {String::literally("orange")}
    else {String::literally("unknown")}
}
