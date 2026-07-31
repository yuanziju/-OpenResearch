// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2023, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ResolvedJavaField`：已解析字段接口。
//!
//! 偏离记录：
//! - Java `ResolvedJavaField extends JavaField, ModifiersProvider, AnnotatedElement, Annotated`
//!   → Rust `trait ResolvedJavaField: JavaField + ModifiersProvider + Annotated`（跳过
//!   `AnnotatedElement`，其方法依赖 Java 反射 `Annotation`/`Class` 类型，Rust 侧为不透明标记，
//!   非本期 T4 范围）。
//! - Java `getDeclaringClass()` 协变覆写（`JavaType` → `ResolvedJavaType`）：Rust 不支持协变返回，
//!   继承 `JavaField::get_declaring_class() -> &dyn JavaType`，并增设 `get_resolved_declaring_class()`
//!   返回 `&dyn ResolvedJavaType`（对齐 `ResolvedJavaType` 的 `resolved_*` 命名约定），供
//!   `SpeculationReasonEncoding::add_field` 等需 `ResolvedJavaType` 的调用方使用。
//! - `getConstantValue()` 默认抛 `UnsupportedOperationException` → Rust 默认 `panic!`。

use crate::meta::annotated::Annotated;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_field::JavaField;
use crate::meta::modifiers_provider::ModifiersProvider;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface ResolvedJavaField extends JavaField, ModifiersProvider, Annotated`。
pub trait ResolvedJavaField: JavaField + ModifiersProvider + Annotated {
    /// 对应 `getOffset()`。
    fn get_offset(&self) -> i32;

    /// 对应 `isFinal()`（默认方法，委托 `isFinalFlagSet`）。
    fn is_final(&self) -> bool {
        self.is_final_flag_set()
    }

    /// 对应 `isInternal()`。
    fn is_internal(&self) -> bool;

    /// 对应 `isSynthetic()`。
    fn is_synthetic(&self) -> bool;

    /// 对应 `getConstantValue()`（默认抛 `UnsupportedOperationException`）。
    fn get_constant_value(&self) -> Box<dyn JavaConstant> {
        panic!("UnsupportedOperationException");
    }

    /// Rust 增设：对应 Java 协变覆写 `getDeclaringClass()`（返回 `ResolvedJavaType`）。
    /// 与 `JavaField::get_declaring_class`（返回 `&dyn JavaType`）并存。
    fn get_resolved_declaring_class(&self) -> &dyn ResolvedJavaType;

    /// Rust 增设：覆写 `JavaField::as_resolved_java_field` 返回 `Some(self)`。
    fn as_resolved_java_field(&self) -> Option<&dyn ResolvedJavaField>
    where
        Self: Sized,
    {
        Some(self)
    }
}
