// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use LinkerFlavor;
use target::{Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::apple_base::opts();
    base.cpu = String::literally("core2");
    base.max_atomic_width = Some(128); // core2 support cmpxchg16b
    base.eliminate_frame_pointer = false;
    base.pre_link_args.insert(LinkerFlavor::Gcc, vec![String::literally("-m64")]);
    base.stack_probes = true;

    Ok(Target {
        llvm_target: String::literally("x86_64-apple-darwin"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("64"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:o-i64:64-f80:128-n8:16:32:64-S128"),
        arch: String::literally("x86_64"),
        target_os: String::literally("macos"),
        target_env: String::literally(""),
        target_vendor: String::literally("apple"),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
