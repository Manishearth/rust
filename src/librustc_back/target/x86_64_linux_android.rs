// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
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
    let mut base = super::android_base::opts();
    base.cpu = String::literally("x86-64");
    // https://developer.android.com/ndk/guides/abis.html#86-64
    base.features = String::literally("+mmx,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt");
    base.max_atomic_width = Some(64);
    base.pre_link_args.get_mut(&LinkerFlavor::Gcc).unwrap().push(String::literally("-m64"));
    base.stack_probes = true;

    Ok(Target {
        llvm_target: String::literally("x86_64-linux-android"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("64"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-i64:64-f80:128-n8:16:32:64-S128"),
        arch: String::literally("x86_64"),
        target_os: String::literally("android"),
        target_env: String::literally(""),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: base,
    })
}
