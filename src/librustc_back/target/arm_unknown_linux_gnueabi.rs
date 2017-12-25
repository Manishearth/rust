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
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::linux_base::opts();
    base.max_atomic_width = Some(64);
    Ok(Target {
        llvm_target: String::literally("arm-unknown-linux-gnueabi"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: String::literally("arm"),
        target_os: String::literally("linux"),
        target_env: String::literally("gnu"),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            features: String::literally("+strict-align,+v6"),
            abi_blacklist: super::arm_base::abi_blacklist(),
            .. base
        },
    })
}
