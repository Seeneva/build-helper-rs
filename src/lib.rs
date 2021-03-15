// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;

pub use cc::*;

mod cc;

pub const ANDROID: &str = "android";

pub trait CmakeExt {
    /// Add target defines
    fn custom_target_define(&mut self) -> &mut Self;
}

impl CmakeExt for cmake::Config {
    fn custom_target_define(&mut self) -> &mut Self {
        let target = env::var("CARGO_CFG_TARGET_OS").unwrap();

        match target.as_str() {
            ANDROID => {
                for env_key in &[
                    "CMAKE_TOOLCHAIN_FILE",
                    "ANDROID_ABI",
                    "ANDROID_NATIVE_API_LEVEL",
                ] {
                    if let Ok(env_val) = env::var(env_key) {
                        self.define(env_key, env_val);
                    }
                }

                self
            }
            _ => self,
        }
    }
}
