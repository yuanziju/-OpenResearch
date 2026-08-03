// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2022, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotResolvedJavaMethod`：HotSpot 已解析方法接口。
//!
//! 偏离记录：
//! - Java `interface HotSpotResolvedJavaMethod extends ResolvedJavaMethod` →
//!   Rust `pub trait HotSpotResolvedJavaMethod: ResolvedJavaMethod`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface HotSpotResolvedJavaMethod extends ResolvedJavaMethod`。
pub trait HotSpotResolvedJavaMethod: ResolvedJavaMethod {
    /// 对应 `boolean isCallerSensitive()`。
    fn is_caller_sensitive(&self) -> bool;

    /// 对应 `boolean isForceInline()`。
    fn is_force_inline(&self) -> bool;

    /// 对应 `boolean hasReservedStackAccess()`。
    fn has_reserved_stack_access(&self) -> bool;

    /// 对应 `void setNotInlinableOrCompilable()`。
    fn set_not_inlinable_or_compilable(&self);

    /// 对应 `boolean ignoredBySecurityStackWalk()`。
    fn ignored_by_security_stack_walk(&self) -> bool;

    /// 对应 `ResolvedJavaMethod uniqueConcreteMethod(HotSpotResolvedObjectType)`。
    fn unique_concrete_method(
        &self,
        receiver: &dyn ResolvedJavaType,
    ) -> Option<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `boolean hasCompiledCode()`。
    fn has_compiled_code(&self) -> bool;

    /// 对应 `boolean hasCompiledCodeAtLevel(int)`。
    fn has_compiled_code_at_level(&self, level: i32) -> bool;

    /// 对应 `int vtableEntryOffset(ResolvedJavaType)`。
    fn vtable_entry_offset(&self, resolved: &dyn ResolvedJavaType) -> i32;

    /// 对应 `int intrinsicId()`。
    fn intrinsic_id(&self) -> i32;

    /// 对应 `boolean isIntrinsicCandidate()`。
    fn is_intrinsic_candidate(&self) -> bool;

    /// 对应 `int allocateCompileId(int)`。
    fn allocate_compile_id(&self, entry_bci: i32) -> i32;

    /// 对应 `boolean hasCodeAtLevel(int, int)`。
    fn has_code_at_level(&self, entry_bci: i32, level: i32) -> bool;

    /// 对应 `int methodIdnum()`。
    fn method_idnum(&self) -> i32;

    /// 对应 `BitSet getOopMapAt(int)`。
    fn get_oop_map_at(&self, bci: i32) -> Vec<u64>;
}
