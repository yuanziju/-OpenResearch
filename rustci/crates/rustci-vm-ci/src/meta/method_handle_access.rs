// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only,
 * as published by the Free Software Foundation.
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

//! 镜像 `jdk.vm.ci.meta.MethodHandleAccessProvider`：`MethodHandle` 内部访问接口。

use crate::meta::java_constant::JavaConstant;
use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// 对应 `MethodHandleAccessProvider.IntrinsicMethod`：`MethodHandle` 上定义的内在方法标识。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum IntrinsicMethod {
    /// 对应 `MethodHandle.invokeBasic`。
    InvokeBasic,
    /// 对应 `MethodHandle.linkToStatic`。
    LinkToStatic,
    /// 对应 `MethodHandle.linkToSpecial`。
    LinkToSpecial,
    /// 对应 `MethodHandle.linkToVirtual`。
    LinkToVirtual,
    /// 对应 `MethodHandle.linkToInterface`。
    LinkToInterface,
    /// 对应 `MethodHandle.linkToNative`。
    LinkToNative,
}

/// 对应 `public interface MethodHandleAccessProvider`。
///
/// 偏离记录：
/// - Java `lookupMethodHandleIntrinsic` 返回 nullable → Rust `Option<IntrinsicMethod>`。
/// - Java `resolveInvokeBasicTarget`/`resolveLinkToTarget` 返回 nullable → Rust `Option`。
/// - Java `NullPointerException`/`IllegalArgumentException`（非受检）→ Rust `panic!`。
pub trait MethodHandleAccessProvider {
    /// 对应 `lookupMethodHandleIntrinsic(ResolvedJavaMethod)`。
    fn lookup_method_handle_intrinsic(
        &self,
        method: &dyn ResolvedJavaMethod,
    ) -> Option<IntrinsicMethod>;

    /// 对应 `resolveInvokeBasicTarget(JavaConstant, boolean)`。
    fn resolve_invoke_basic_target(
        &self,
        method_handle: &dyn JavaConstant,
        force_bytecode_generation: bool,
    ) -> Option<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `resolveLinkToTarget(JavaConstant)`。
    fn resolve_link_to_target(
        &self,
        member_name: &dyn JavaConstant,
    ) -> Option<Box<dyn ResolvedJavaMethod>>;
}
