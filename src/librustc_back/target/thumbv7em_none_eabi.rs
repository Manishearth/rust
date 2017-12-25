// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Targets the Cortex-M4 and Cortex-M7 processors (ARMv7E-M)
//
// This target assumes that the device doesn't have a FPU (Floating Point Unit) and lowers all the
// floating point operations to software routines (intrinsics).
//
// As such, this target uses the "soft" calling convention (ABI) where floating point values are
// passed to/from subroutines via general purpose registers (R0, R1, etc.).
//
// To opt-in to hardware accelerated floating point operations, you can use, for example,
// `-C target-feature=+vfp4` or `-C target-cpu=cortex-m4`.

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: String::literally("thumbv7em-none-eabi"),
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
            max_atomic_width: Some(32),
            .. super::thumb_base::opts()
        },
    })
}
