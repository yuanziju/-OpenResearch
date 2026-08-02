// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2025, Oracle and/or its affiliates. All rights reserved.
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
 * or visit oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.code.CodeCacheProvider`：代码缓存相关细节与需求的访问接口。
//!
//! 偏离记录：
//! - Java `interface CodeCacheProvider` → Rust `pub trait CodeCacheProvider`。`addCode`/`setDefaultCode`/
//!   `getMarkName`/`getTargetName` 默认方法保留为 trait 默认实现。
//! - `method` 为 `Box<dyn ResolvedJavaMethod>`（Java 持引用，Rust 转移所有权供 impl 安装）。
//! - `log`/`installedCode` nullable → `Option<Box<...>>`。`InstalledCode` 在 Rust 侧为 struct（非 trait，
//!   见 `installed_code.rs` 偏离），故 `installCode` 返回 `Box<InstalledCode>` 而非
//!   `Box<dyn InstalledCode>`（任务规格写法）。
//! - `getTarget` 返回 owned `TargetDescription`（任务规格 `-> TargetDescription`）。`Architecture`
//!   含 `Box<dyn ...>` 非 `Clone`，impl 须每次构造或经共享指针提供（T7 绑定具体后端时处理）。
//! - `getMarkName` 默认 `String.valueOf(mark.id)` → `format!("{:?}", mark.id)`：`mark.id` 为
//!   `Box<dyn Debug>`（见 `site::Mark` 偏离），`{:?}` 对齐 `String.valueOf` 的字符串化语义。
//! - `getTargetName` 默认 `String.valueOf(call.target)` → `format!("{:?}", call.target)`：
//!   `InvokeTarget: Debug`，`Box<dyn InvokeTarget>: Debug`。

use crate::code::compiled_code::CompiledCode;
use crate::code::installed_code::InstalledCode;
use crate::code::register_config::RegisterConfig;
use crate::code::site::{Call, Mark};
use crate::code::target_description::TargetDescription;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::speculation_log::SpeculationLog;

/// 对应 `interface CodeCacheProvider`。
pub trait CodeCacheProvider {
    /// 对应 `default InstalledCode addCode(ResolvedJavaMethod, CompiledCode, SpeculationLog, InstalledCode)`。
    fn add_code(
        &self,
        method: Box<dyn ResolvedJavaMethod>,
        compiled_code: Box<dyn CompiledCode>,
        log: Option<Box<dyn SpeculationLog>>,
        installed_code: Option<Box<InstalledCode>>,
    ) -> Box<InstalledCode> {
        self.install_code(method, compiled_code, installed_code, log, false)
    }

    /// 对应 `default InstalledCode setDefaultCode(ResolvedJavaMethod, CompiledCode)`。
    fn set_default_code(
        &self,
        method: Box<dyn ResolvedJavaMethod>,
        compiled_code: Box<dyn CompiledCode>,
    ) -> Box<InstalledCode> {
        self.install_code(method, compiled_code, None, None, true)
    }

    /// 对应 `InstalledCode installCode(ResolvedJavaMethod, CompiledCode, InstalledCode, SpeculationLog, boolean)`。
    fn install_code(
        &self,
        method: Box<dyn ResolvedJavaMethod>,
        compiled_code: Box<dyn CompiledCode>,
        installed_code: Option<Box<InstalledCode>>,
        log: Option<Box<dyn SpeculationLog>>,
        is_default: bool,
    ) -> Box<InstalledCode>;

    /// 对应 `void invalidateInstalledCode(InstalledCode)`。
    fn invalidate_installed_code(&self, installed_code: &InstalledCode);

    /// 对应 `default String getMarkName(Mark mark)`：`String.valueOf(mark.id)`。
    fn get_mark_name(&self, mark: &Mark) -> String {
        format!("{:?}", mark.id)
    }

    /// 对应 `default String getTargetName(Call call)`：`String.valueOf(call.target)`。
    fn get_target_name(&self, call: &Call) -> String {
        format!("{:?}", call.target)
    }

    /// 对应 `RegisterConfig getRegisterConfig()`。
    fn get_register_config(&self) -> Box<dyn RegisterConfig>;

    /// 对应 `int getMinimumOutgoingSize()`。
    fn get_minimum_outgoing_size(&self) -> i32;

    /// 对应 `TargetDescription getTarget()`。
    fn get_target(&self) -> TargetDescription;

    /// 对应 `SpeculationLog createSpeculationLog()`。
    fn create_speculation_log(&self) -> Box<dyn SpeculationLog>;

    /// 对应 `long getMaxCallTargetOffset(long address)`。
    fn get_max_call_target_offset(&self, address: i64) -> i64;

    /// 对应 `boolean shouldDebugNonSafepoints()`。
    fn should_debug_non_safepoints(&self) -> bool;
}
