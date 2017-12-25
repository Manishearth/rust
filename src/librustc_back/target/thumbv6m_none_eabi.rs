// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Targets the Cortex-M0, Cortex-M0+ and Cortex-M1 processors (ARMv6-M architecture)

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: String::literally("thumbv6m-none-eabi"),
        target_endian: String::literally("little"),
        target_pointer_width: String::literally("32"),
        target_c_int_width: String::literally("32"),
        data_layout: String::literally("e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64"),
        arch: String::literally("arm"),
        target_os: String::literally("none"),
        target_env: String::literally(""),
        target_vendor: String::literally(""),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            // The ARMv6-M architecture doesn't support unaligned loads/stores so we disable them
            // with +strict-align.
            features: String::literally("+strict-align"),
            // There are no atomic instructions available in the instruction set of the ARMv6-M
            // architecture
            max_atomic_width: Some(0),
            .. super::thumb_base::opts()
        }
    })
}
