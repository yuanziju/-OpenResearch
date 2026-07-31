// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ConstantReflectionProvider`：常量的反射操作接口。
//!
//! 偏离记录：
//! - Java `constantEquals` 返回 `Boolean`（nullable）→ Rust `Option<bool>`。
//! - Java `readArrayLength` 返回 `Integer`（nullable）→ Rust `Option<i32>`。
//! - Java `readArrayElement`/`readFieldValue`/`boxPrimitive`/`unboxPrimitive` 返回 nullable
//!   `JavaConstant` → Rust `Option<Box<dyn JavaConstant>>`。
//! - Java `asJavaType` 返回 nullable `ResolvedJavaType` → Rust `Option<Box<dyn ResolvedJavaType>>`。
//! - Java `asObjectHub` 返回 `Constant` → Rust `Box<dyn Constant>`。
//! - Java `getMethodHandleAccess`/`getMemoryAccessProvider` 返回接口引用 → Rust `&dyn ...`。

use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::memory_access::MemoryAccessProvider;
use crate::meta::method_handle_access::MethodHandleAccessProvider;
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `public interface ConstantReflectionProvider`。
pub trait ConstantReflectionProvider {
    /// 对应 `constantEquals(Constant, Constant)`：返回 `Option<bool>` 对齐 Java nullable `Boolean`。
    fn constant_equals(&self, x: &dyn Constant, y: &dyn Constant) -> Option<bool>;

    /// 对应 `readArrayLength(JavaConstant)`：返回 `Option<i32>` 对齐 Java nullable `Integer`。
    fn read_array_length(&self, array: &dyn JavaConstant) -> Option<i32>;

    /// 对应 `readArrayElement(JavaConstant, int)`：返回 `Option` 对齐 Java nullable。
    fn read_array_element(
        &self,
        array: &dyn JavaConstant,
        index: i32,
    ) -> Option<Box<dyn JavaConstant>>;

    /// 对应 `readFieldValue(ResolvedJavaField, JavaConstant)`：返回 `Option` 对齐 Java nullable。
    fn read_field_value(
        &self,
        field: &dyn ResolvedJavaField,
        receiver: &dyn JavaConstant,
    ) -> Option<Box<dyn JavaConstant>>;

    /// 对应 `boxPrimitive(JavaConstant)`：返回 `Option` 对齐 Java nullable。
    fn box_primitive(&self, source: &dyn JavaConstant) -> Option<Box<dyn JavaConstant>>;

    /// 对应 `unboxPrimitive(JavaConstant)`：返回 `Option` 对齐 Java nullable。
    fn unbox_primitive(&self, source: &dyn JavaConstant) -> Option<Box<dyn JavaConstant>>;

    /// 对应 `forString(String)`。
    fn for_string(&self, value: &str) -> Box<dyn JavaConstant>;

    /// 对应 `asJavaType(Constant)`：返回 `Option` 对齐 Java nullable。
    fn as_java_type(&self, constant: &dyn Constant) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `getMethodHandleAccess()`。
    fn get_method_handle_access(&self) -> &dyn MethodHandleAccessProvider;

    /// 对应 `getMemoryAccessProvider()`。
    fn get_memory_access_provider(&self) -> &dyn MemoryAccessProvider;

    /// 对应 `asJavaClass(ResolvedJavaType)`。
    fn as_java_class(&self, type_: &dyn ResolvedJavaType) -> Box<dyn JavaConstant>;

    /// 对应 `asObjectHub(ResolvedJavaType)`：返回 `Box<dyn Constant>`。
    fn as_object_hub(&self, type_: &dyn ResolvedJavaType) -> Box<dyn Constant>;
}
