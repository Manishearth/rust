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
    let mut base = super::dragonfly_base::opts();
    base.cpu = String::literally("pentium4");
    base.max_atomic_width = Some(64);
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push(String::literally("-m32"));
    base.stack_probes = true;

    Ok(Target {
        llvm_target: String::literally("i686-unknown-dragonfly"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-p:32:32-f64:32:64-f80:32-n8:16:32-S128"),
        arch: String::literally("x86"),
        target_os: String::literally("dragonfly"),
        target_env: String::literally(""),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
