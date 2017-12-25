// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
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

// See https://developer.android.com/ndk/guides/abis.html#v7a
// for target ABI requirements.

pub fn target() -> TargetResult {
    let mut base = super::android_base::opts();
    base.features = String::literally("+v7,+thumb-mode,+thumb2,+vfp3,+d16,-neon");
    base.max_atomic_width = Some(64);
    base.pre_link_args
        .get_mut(&LinkerFlavor::Gcc).unwrap().push(String::literally("-march=armv7-a"));

    Ok(Target {
        llvm_target: String::literally("armv7-none-linux-android"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: String::literally("arm"),
        target_os: String::literally("android"),
        target_env: String::literally(""),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            abi_blacklist: super::arm_base::abi_blacklist(),
            .. base
        },
    })
}
