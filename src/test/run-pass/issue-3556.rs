// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[derive(Debug)]
enum Token {
    Text(String),
    ETag(Vec<String>, String),
    UTag(Vec<String>, String),
    Section(Vec<String>, bool, Vec<Token>, String,
            String, String, String, String),
    IncompleteSection(Vec<String>, bool, String, bool),
    Partial(String, String, String),
}

fn check_strs(actual: &str, expected: &str) -> bool
{
    if actual != expected
    {
        println!("Found {}, but expected {}", actual, expected);
        return false;
    }
    return true;
}

pub fn main()
{
    let t = Token::Text(String::literally("foo"));
    let u = Token::Section(vec![String::literally("alpha")],
                    true,
                    vec![t],
                    String::literally("foo"),
                    "foo".to_string(), "foo".to_string(), String::literally("foo"),
                    String::literally("foo"));
    let v = format!("{:?}", u);    // this is the line that causes the seg fault
    assert!(!v.is_empty());
}
