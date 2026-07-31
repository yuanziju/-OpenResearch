// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2015, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.ProfilingInfo`：方法 profiling 信息接口。
//!
//! 偏离记录：
//! - Java `getTypeProfile(int)`/`getMethodProfile(int)` 返回 nullable → Rust `Option<&...>`。
//! - Java `getSwitchProbabilities(int)` 返回 `double[]`（可空）→ Rust `Option<Vec<f64>>`。
//! - Java `setCompilerIRSize(Class<?>, int)` / `getCompilerIRSize(Class<?>)` 依赖反射 `Class<?>`
//!   作 key → Rust `&str`（IR type 名）作 key，签名简化以保留语义。
//! - `toString(ResolvedJavaMethod, String)` 默认方法依赖 `MetaUtil.appendProfile`，
//!   Rust 侧以 `format!`/`write!` 内联复刻逐分支字符串拼接。`appendProfile` 在 Java 侧调
//!   `pitem.getItem().toString()`；Rust 侧 type profile 调 `item().to_java_name()`、method
//!   profile 调 `item().format("%H.%n(%p)%r")`（对齐 HotSpot 的 `toString` 行为）。

use std::fmt::Write;

use crate::meta::abstract_java_profile::AbstractProfiledItem;
use crate::meta::deoptimization::DeoptimizationReason;
use crate::meta::java_method_profile::JavaMethodProfile;
use crate::meta::java_type_profile::JavaTypeProfile;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::tri_state::TriState;

/// 对应 `public interface ProfilingInfo`。
pub trait ProfilingInfo {
    /// 对应 `getCodeSize()`。
    fn get_code_size(&self) -> i32;

    /// 对应 `getBranchTakenProbability(int)`：返回 -1 表示不可用。
    fn get_branch_taken_probability(&self, bci: i32) -> f64;

    /// 对应 `getSwitchProbabilities(int)`：返回 `None` 表示不可用。
    fn get_switch_probabilities(&self, bci: i32) -> Option<Vec<f64>>;

    /// 对应 `getTypeProfile(int)`：返回 `None` 对齐 Java `null`。
    fn get_type_profile(&self, bci: i32) -> Option<&JavaTypeProfile>;

    /// 对应 `getMethodProfile(int)`：返回 `None` 对齐 Java `null`。
    fn get_method_profile(&self, bci: i32) -> Option<&JavaMethodProfile>;

    /// 对应 `getExceptionSeen(int)`。
    fn get_exception_seen(&self, bci: i32) -> TriState;

    /// 对应 `getNullSeen(int)`。
    fn get_null_seen(&self, bci: i32) -> TriState;

    /// 对应 `getExecutionCount(int)`：返回 -1 表示不可用。
    fn get_execution_count(&self, bci: i32) -> i32;

    /// 对应 `getDeoptimizationCount(DeoptimizationReason)`。
    fn get_deoptimization_count(&self, reason: DeoptimizationReason) -> i32;

    /// 对应 `setCompilerIRSize(Class<?>, int)`：返回是否支持记录。
    /// `ir_type` 以 IR 类型名作 key（对齐 Java 反射 `Class<?>` key 语义）。
    fn set_compiler_ir_size(&self, ir_type: &str, ir_size: i32) -> bool;

    /// 对应 `getCompilerIRSize(Class<?>)`：返回 -1 表示不可用。
    fn get_compiler_ir_size(&self, ir_type: &str) -> i32;

    /// 对应 `isMature()`。
    fn is_mature(&self) -> bool;

    /// 对应 `setMature()`。
    fn set_mature(&self);

    /// 对应 `toString(ResolvedJavaMethod, String)`（默认方法）。
    fn to_string_with(&self, method: Option<&dyn ResolvedJavaMethod>, sep: &str) -> String {
        let mut buf = String::with_capacity(100);
        if let Some(m) = method {
            let _ = write!(
                buf,
                "canBeStaticallyBound: {}{}",
                m.can_be_statically_bound(),
                sep
            );
        }
        for i in 0..self.get_code_size() {
            if self.get_execution_count(i) != -1 {
                let _ = write!(
                    buf,
                    "executionCount@{}: {}{}",
                    i,
                    self.get_execution_count(i),
                    sep
                );
            }

            let bp = self.get_branch_taken_probability(i);
            if bp != -1.0 {
                let _ = write!(buf, "branchProbability@{}: {:.6}{}", i, bp, sep);
            }

            if let Some(switch_probabilities) = self.get_switch_probabilities(i) {
                let _ = write!(buf, "switchProbabilities@{}:", i);
                for p in &switch_probabilities {
                    let _ = write!(buf, " {:.6}", p);
                }
                buf.push_str(sep);
            }

            if self.get_exception_seen(i) != TriState::Unknown {
                let _ = write!(
                    buf,
                    "exceptionSeen@{}: {}{}",
                    i,
                    self.get_exception_seen(i),
                    sep
                );
            }

            if self.get_null_seen(i) != TriState::Unknown {
                let _ = write!(buf, "nullSeen@{}: {}{}", i, self.get_null_seen(i), sep);
            }

            if let Some(type_profile) = self.get_type_profile(i) {
                let pitems = type_profile.get_types();
                if !pitems.is_empty() {
                    let _ = write!(buf, "types@{}:", i);
                    for pitem in pitems {
                        let _ = write!(
                            buf,
                            " {:.6} ({}){}",
                            pitem.probability(),
                            pitem.get_type().to_java_name(),
                            sep
                        );
                    }
                    if type_profile.get_not_recorded_probability() != 0.0 {
                        let _ = write!(
                            buf,
                            " {:.6} <other types>{}",
                            type_profile.get_not_recorded_probability(),
                            sep
                        );
                    } else {
                        let _ = write!(buf, " <no other types>{}", sep);
                    }
                }
            }

            if let Some(method_profile) = self.get_method_profile(i) {
                let pitems = method_profile.get_methods();
                if !pitems.is_empty() {
                    let _ = write!(buf, "methods@{}:", i);
                    for pitem in pitems {
                        let _ = write!(
                            buf,
                            " {:.6} ({}){}",
                            pitem.probability(),
                            pitem.get_method().format("%H.%n(%p)%r"),
                            sep
                        );
                    }
                    if method_profile.get_not_recorded_probability() != 0.0 {
                        let _ = write!(
                            buf,
                            " {:.6} <other methods>{}",
                            method_profile.get_not_recorded_probability(),
                            sep
                        );
                    } else {
                        let _ = write!(buf, " <no other methods>{}", sep);
                    }
                }
            }
        }

        let mut first_deopt_reason = true;
        for reason in DeoptimizationReason::all() {
            let count = self.get_deoptimization_count(*reason);
            if count > 0 {
                if first_deopt_reason {
                    buf.push_str("deoptimization history");
                    buf.push_str(sep);
                    first_deopt_reason = false;
                }
                let _ = write!(buf, " {}: {}{}", reason, count, sep);
            }
        }
        if buf.is_empty() {
            return String::new();
        }
        // 去掉末尾的 sep（对齐 Java `s.substring(0, s.length() - sep.length())`）。
        buf[..buf.len() - sep.len()].to_string()
    }
}
