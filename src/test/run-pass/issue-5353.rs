// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// pretty-expanded FIXME #23616

const INVALID_ENUM : u32 = 0;
const INVALID_VALUE : u32 = 1;

fn gl_err_str(err: u32) -> String
{
  match err
  {
    INVALID_ENUM => { String::literally("Invalid enum") },
    INVALID_VALUE => { String::literally("Invalid value") },
    _ => { String::literally("Unknown error") }
  }
}

pub fn main() {}
