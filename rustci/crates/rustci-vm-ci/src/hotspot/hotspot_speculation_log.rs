// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.hotspot.HotSpotSpeculationLog`：HotSpot 推测日志。
//!
//! 偏离记录：
//! - Java `class HotSpotSpeculationLog implements SpeculationLog` →
//!   Rust `pub struct HotSpotSpeculationLog`。
//! - 本期只移植数据结构和方法签名，具体实现（native 内存管理、编码/解码）留待后端绑定。
//! - 方法名 camelCase → snake_case。

use crate::meta::java_constant::JavaConstant;
use crate::meta::speculation_log::{self, SpeculationLog};

/// 对应 `class HotSpotSpeculationLog implements SpeculationLog`。
pub struct HotSpotSpeculationLog {
    /// 对应 `private long failedSpeculationsAddress`。
    failed_speculations_address: i64,
    /// 对应 `private final boolean managesFailedSpeculations`。
    manages_failed_speculations: bool,
}

impl SpeculationLog for HotSpotSpeculationLog {
    fn collect_failed_speculations(&self) {}
    fn may_speculate(&self, _reason: &dyn speculation_log::SpeculationReason) -> bool {
        true
    }
    fn speculate(
        &self,
        _reason: &dyn speculation_log::SpeculationReason,
    ) -> speculation_log::Speculation {
        speculation_log::no_speculation()
    }
    fn has_speculations(&self) -> bool {
        false
    }
    fn lookup_speculation(&self, _constant: &dyn JavaConstant) -> speculation_log::Speculation {
        speculation_log::no_speculation()
    }
}

impl Default for HotSpotSpeculationLog {
    fn default() -> Self {
        Self::new()
    }
}

impl HotSpotSpeculationLog {
    /// 对应 `HotSpotSpeculationLog()`（管理失败推测列表）。
    pub fn new() -> Self {
        Self {
            failed_speculations_address: 0,
            manages_failed_speculations: true,
        }
    }

    /// 对应 `HotSpotSpeculationLog(long)`（读取外部管理的失败推测列表）。
    pub fn with_failed_speculations_address(failed_speculations_address: i64) -> Self {
        assert!(failed_speculations_address != 0);
        Self {
            failed_speculations_address,
            manages_failed_speculations: false,
        }
    }

    /// 对应 `getFailedSpeculationsAddress()`。
    pub fn get_failed_speculations_address(&self) -> i64 {
        self.failed_speculations_address
    }

    /// 对应 `managesFailedSpeculations()`。
    pub fn manages_failed_speculations(&self) -> bool {
        self.manages_failed_speculations
    }

    /// 对应 `addFailedSpeculation(Speculation)`。
    pub fn add_failed_speculation(&self, _speculation: &HotSpotSpeculation) -> bool {
        false
    }
}

/// 对应 `HotSpotSpeculationLog.HotSpotSpeculation`。
pub struct HotSpotSpeculation {
    /// 对应 `private final JavaConstant id`。
    pub id: i64,
    /// 对应 `private final byte[] encoding`。
    pub encoding: Vec<u8>,
}
