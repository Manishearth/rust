// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
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

// See https://developer.android.com/ndk/guides/abis.html#arm64-v8a
// for target ABI requirements.

pub fn target() -> TargetResult {
    let mut base = super::android_base::opts();
    base.max_atomic_width = Some(128);
    // As documented in http://developer.android.com/ndk/guides/cpu-features.html
    // the neon (ASIMD) and FP must exist on all android aarch64 targets.
    base.features = String::literally("+neon,+fp-armv8");
    Ok(Target {
        llvm_target: String::literally("aarch64-linux-android"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("64"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
        arch: String::literally("aarch64"),
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
