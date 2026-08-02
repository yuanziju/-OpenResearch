// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2025, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.stack`：栈内省（stack introspection）接口。
//!
//! 偏离记录：
//! - Java `interface InspectedFrame` → Rust `pub trait InspectedFrame`。`getLocal(int)` 返回 `Object`
//!   → `Box<dyn Any>`（Java 持有引用，Rust 返回 owned 以表达「值或 null」；null 由 `as_any` 下转失败
//!   表达，与 `Object` 语义一致）。`getMethod()` 返回 `ResolvedJavaMethod`（引用）→
//!   `&dyn ResolvedJavaMethod`（偏离任务规格 `Box<dyn ResolvedJavaMethod>`：`ResolvedJavaMethod` 无
//!   `Clone`，无法返回 owned `Box` 而无 `Arc`/共享指针；引用对齐 Java 语义）。
//! - `InspectedFrameVisitor<T>`：Java `T visitFrame(InspectedFrame)`（null=继续，非 null=停止）→
//!   Rust `Option<T>`（`None`=继续，`Some`=停止），对齐 Java null 语义。
//! - `StackIntrospection.iterateFrames<T>`：Java `<T> T iterateFrames(...)` 泛型方法 → Rust 泛型方法
//!   + `where Self: Sized` 约束（与 `meta::Value::get_value_kind_as` 一致），使 trait 保持 dyn 兼容。
//!   `initialMethods`/`matchingMethods` 的 `ResolvedJavaMethod[]`（引用数组）→ `&[&dyn ResolvedJavaMethod]`。

use std::any::Any;

use crate::meta::resolved_java_method::ResolvedJavaMethod;

/// 对应 `interface InspectedFrame`。
pub trait InspectedFrame {
    /// 对应 `Object getLocal(int index)`。
    fn get_local(&self, index: i32) -> Box<dyn Any>;

    /// 对应 `boolean isVirtual(int index)`。
    fn is_virtual(&self, index: i32) -> bool;

    /// 对应 `boolean hasVirtualObjects()`。
    fn has_virtual_objects(&self) -> bool;

    /// 对应 `void materializeVirtualObjects(boolean invalidateCode)`。
    fn materialize_virtual_objects(&self, invalidate_code: bool);

    /// 对应 `int getBytecodeIndex()`。
    fn get_bytecode_index(&self) -> i32;

    /// 对应 `ResolvedJavaMethod getMethod()`。
    fn get_method(&self) -> &dyn ResolvedJavaMethod;

    /// 对应 `boolean isMethod(ResolvedJavaMethod method)`：语义等价 `method.equals(getMethod())`。
    fn is_method(&self, method: &dyn ResolvedJavaMethod) -> bool;
}

/// 对应 `interface InspectedFrameVisitor<T>`。
///
/// `visitFrame` 返回 `None` 表示继续遍历下一 caller 帧，返回 `Some` 表示停止。
pub trait InspectedFrameVisitor<T> {
    /// 对应 `T visitFrame(InspectedFrame frame)`。
    fn visit_frame(&self, frame: &dyn InspectedFrame) -> Option<T>;
}

/// 对应 `interface StackIntrospection`。
pub trait StackIntrospection {
    /// 对应 `<T> T iterateFrames(ResolvedJavaMethod[] initialMethods, ResolvedJavaMethod[] matchingMethods,
    /// int initialSkip, InspectedFrameVisitor<T> visitor)`。
    ///
    /// `where Self: Sized` 使本泛型方法不进入 vtable，trait 保持 dyn 兼容（对齐 `meta::Value` 约定）。
    fn iterate_frames<T>(
        &self,
        initial_methods: &[&dyn ResolvedJavaMethod],
        matching_methods: &[&dyn ResolvedJavaMethod],
        initial_skip: i32,
        visitor: &dyn InspectedFrameVisitor<T>,
    ) -> Option<T>
    where
        Self: Sized;

    /// 对应 `default boolean canMaterializeVirtualObjects()`。
    fn can_materialize_virtual_objects(&self) -> bool {
        true
    }
}
