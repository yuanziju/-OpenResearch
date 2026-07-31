// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2025, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES IN THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit Oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.DefaultProfilingInfo`：无真实 profile 信息时的占位实现。
//!
//! 偏离记录：
//! - Java `final class DefaultProfilingInfo implements ProfilingInfo` → Rust `pub struct
//!   DefaultProfilingInfo` 实现 `ProfilingInfo` trait。
//! - Java `static final ProfilingInfo[] NO_PROFILING_INFO` 缓存 3 实例（按 `TriState.ordinal()`
//!   索引）→ Rust 侧 `std::sync::OnceLock<DefaultProfilingInfo>` 静态缓存 3 实例，`get` 按
//!   `TriState` 变体返回 `&'static` 引用（对齐 Java `get` 返回同一缓存实例语义）。
//! - Java `toString()` 调 `this.toString(null, "; ")`（`ProfilingInfo.toString` 默认方法）→
//!   Rust 侧 `Display` 调 `self.to_string_with(None, "; ")`。

use std::fmt;
use std::sync::OnceLock;

use crate::meta::deoptimization::DeoptimizationReason;
use crate::meta::java_method_profile::JavaMethodProfile;
use crate::meta::java_type_profile::JavaTypeProfile;
use crate::meta::profiling_info::ProfilingInfo;
use crate::meta::tri_state::TriState;

/// 对应 `public final class DefaultProfilingInfo implements ProfilingInfo`。
pub struct DefaultProfilingInfo {
    exception_seen: TriState,
}

static TRUE_INSTANCE: OnceLock<DefaultProfilingInfo> = OnceLock::new();
static FALSE_INSTANCE: OnceLock<DefaultProfilingInfo> = OnceLock::new();
static UNKNOWN_INSTANCE: OnceLock<DefaultProfilingInfo> = OnceLock::new();

impl DefaultProfilingInfo {
    /// 对应 `DefaultProfilingInfo(TriState exceptionSeen)`（Java 包私有构造器）。
    pub fn new(exception_seen: TriState) -> Self {
        Self { exception_seen }
    }

    /// 对应 `static ProfilingInfo get(TriState exceptionSeen)`：返回缓存实例。
    pub fn get(exception_seen: TriState) -> &'static DefaultProfilingInfo {
        match exception_seen {
            TriState::True => {
                TRUE_INSTANCE.get_or_init(|| DefaultProfilingInfo::new(TriState::True))
            }
            TriState::False => {
                FALSE_INSTANCE.get_or_init(|| DefaultProfilingInfo::new(TriState::False))
            }
            TriState::Unknown => {
                UNKNOWN_INSTANCE.get_or_init(|| DefaultProfilingInfo::new(TriState::Unknown))
            }
        }
    }
}

impl ProfilingInfo for DefaultProfilingInfo {
    fn get_code_size(&self) -> i32 {
        0
    }

    fn get_branch_taken_probability(&self, _bci: i32) -> f64 {
        -1.0
    }

    fn get_switch_probabilities(&self, _bci: i32) -> Option<Vec<f64>> {
        None
    }

    fn get_type_profile(&self, _bci: i32) -> Option<&JavaTypeProfile> {
        None
    }

    fn get_method_profile(&self, _bci: i32) -> Option<&JavaMethodProfile> {
        None
    }

    fn get_exception_seen(&self, _bci: i32) -> TriState {
        self.exception_seen
    }

    fn get_null_seen(&self, _bci: i32) -> TriState {
        TriState::Unknown
    }

    fn get_execution_count(&self, _bci: i32) -> i32 {
        -1
    }

    fn get_deoptimization_count(&self, _reason: DeoptimizationReason) -> i32 {
        0
    }

    fn set_compiler_ir_size(&self, _ir_type: &str, _ir_size: i32) -> bool {
        false
    }

    fn get_compiler_ir_size(&self, _ir_type: &str) -> i32 {
        -1
    }

    fn is_mature(&self) -> bool {
        false
    }

    fn set_mature(&self) {
        // 对应 Java `// Do nothing`。
    }
}

impl fmt::Display for DefaultProfilingInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`"DefaultProfilingInfo<" + this.toString(null, "; ") + ">"`。
        write!(
            f,
            "DefaultProfilingInfo<{}>",
            self.to_string_with(None, "; ")
        )
    }
}

impl fmt::Debug for DefaultProfilingInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
