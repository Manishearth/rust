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
    Ok(Target {
        llvm_target: String::literally("mipsel-unknown-linux-musl"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64"),
        arch: String::literally("mips"),
        target_os: String::literally("linux"),
        target_env: String::literally("musl"),
        target_vendor: String::literally("unknown"),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            cpu: String::literally("mips32"),
            features: String::literally("+mips32,+soft-float"),
            max_atomic_width: Some(32),

            // see #36994
            exe_allocation_crate: None,

            ..super::linux_base::opts()
        }
    })
}
