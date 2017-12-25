// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Targets the Cortex-M4F and Cortex-M7F processors (ARMv7E-M)
//
// This target assumes that the device does have a FPU (Floating Point Unit) and lowers all (single
// precision) floating point operations to hardware instructions.
//
// Additionally, this target uses the "hard" floating convention (ABI) where floating point values
// are passed to/from subroutines via FPU registers (S0, S1, D0, D1, etc.).
//
// To opt into double precision hardware support, use the `-C target-feature=-fp-only-sp` flag.

use LinkerFlavor;
use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: String::literally("thumbv7em-none-eabihf"),
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
            // `+vfp4` is the lowest common denominator between the Cortex-M4 (vfp4-16) and the
            // Cortex-M7 (vfp5)
            // `+d16` both the Cortex-M4 and the Cortex-M7 only have 16 double-precision registers
            // available
            // `+fp-only-sp` The Cortex-M4 only supports single precision floating point operations
            // whereas in the Cortex-M7 double precision is optional
            //
            // Reference:
            // ARMv7-M Architecture Reference Manual - A2.5 The optional floating-point extension
            features: String::literally("+vfp4,+d16,+fp-only-sp"),
            max_atomic_width: Some(32),
            .. super::thumb_base::opts()
        }
    })
}
