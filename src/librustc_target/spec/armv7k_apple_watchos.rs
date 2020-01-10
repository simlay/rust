use super::apple_watchos_base::{opts, Arch};
use crate::spec::{LinkerFlavor, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    let base = opts(Arch::Armv7k)?;
    Ok(Target {
        llvm_target: "thumbv7k-apple-watchos".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:o-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "armv7k".to_string(),
        target_os: "watchos".to_string(),
        target_env: String::new(),
        target_vendor: "apple".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: TargetOptions {
            features: "+neon,+fp-armv8,+cyclone".to_string(),
            eliminate_frame_pointer: false,
            max_atomic_width: Some(128),
            abi_blacklist: super::arm_base::abi_blacklist(),
            ..base
        },
    })
}

