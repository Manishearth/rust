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
use super::apple_ios_base::{opts, Arch};

pub fn target() -> TargetResult {
    let base = opts(Arch::Arm64)?;
    Ok(Target {
        llvm_target: String::literally("arm64-apple-ios"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("64"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:o-i64:64-i128:128-n32:64-S128"),
        arch: String::literally("aarch64"),
        target_os: String::literally("ios"),
        target_env: String::literally(""),
        target_vendor: String::literally("apple"),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            features: String::literally("+neon,+fp-armv8,+cyclone"),
            eliminate_frame_pointer: false,
            max_atomic_width: Some(128),
            abi_blacklist: super::arm_base::abi_blacklist(),
            .. base
        },
    })
}
