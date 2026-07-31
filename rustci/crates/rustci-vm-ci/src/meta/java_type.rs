// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2016, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.JavaType`：已解析或未解析类型的抽象。
//!
//! 偏离记录：
//! - `Debug` 超 trait：Java `JavaType` 无 `toString`，但 Rust 侧多类（`EnumData`/
//!   `AnnotationData`、`JavaObjectValue` 等）持 `Box<dyn JavaType>` 并需 `Debug`，
//!   故增设 `Debug` 超 trait（与既有 `JavaValue: Debug` 一致）。
//! - `resolve` 抛 `LinkageError`/`NullPointerException`（Java 非受检），Rust 侧对齐
//!   运行时异常语义 → `panic!`。
//! - `toJavaName()` / `toJavaName(boolean)` 重载 → `to_java_name()` / `to_java_name_qualified(bool)`。
//! - nullable 引用返回（`getComponentType`）→ `Option<&dyn JavaType>`；`getArrayClass`/
//!   `resolve` 为产生值 → `Box<dyn JavaType>` / `Box<dyn ResolvedJavaType>`。

use std::fmt::Debug;

use crate::meta::java_kind::JavaKind;
use crate::meta::meta_util::internal_name_to_java;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface JavaType`。
pub trait JavaType: Debug {
    /// 对应 `getName()`：internal form（如 `"Ljava/lang/Object;"`、`"I"`、`"[[B"`）。
    fn get_name(&self) -> &str;

    /// 对应 `getUnqualifiedName()`。
    fn get_unqualified_name(&self) -> String {
        let mut name = self.get_name().to_string();
        if let Some(idx) = name.rfind('/') {
            name = name[idx + 1..].to_string();
        }
        if name.ends_with(';') {
            name.pop();
        }
        name
    }

    /// 对应 `isArray()`。
    fn is_array(&self) -> bool {
        self.get_component_type().is_some()
    }

    /// 对应 `getComponentType()`：数组元素类型，非数组返回 `None`。
    fn get_component_type(&self) -> Option<&dyn JavaType>;

    /// 对应 `getElementalType()`：零维元素类型。
    ///
    /// 偏离记录：增设 `Self: Sized` 约束——方法体需将 `&Self` 转 `&dyn JavaType`（trait 对象），
    /// Rust 要求 `Self: Sized` 方可进行此 unsized 强转。与 `ResolvedJavaType::get_elemental_type_resolved`
    /// 等同模式。
    fn get_elemental_type(&self) -> &dyn JavaType
    where
        Self: Sized,
    {
        let mut t: &dyn JavaType = self;
        while let Some(c) = t.get_component_type() {
            t = c;
        }
        t
    }

    /// 对应 `getArrayClass()`：以本类型为元素类型的数组类型。
    fn get_array_class(&self) -> Box<dyn JavaType>;

    /// 对应 `getJavaKind()`。
    fn get_java_kind(&self) -> JavaKind;

    /// 对应 `resolve(ResolvedJavaType accessingClass)`。
    fn resolve(&self, accessing_class: &dyn ResolvedJavaType) -> Box<dyn ResolvedJavaType>;

    /// 对应 `toJavaName()`。
    fn to_java_name(&self) -> String {
        internal_name_to_java(self.get_name(), true, false)
    }

    /// 对应 `toJavaName(boolean qualified)`。
    fn to_java_name_qualified(&self, qualified: bool) -> String {
        let kind = self.get_java_kind();
        if kind == JavaKind::Object {
            internal_name_to_java(self.get_name(), qualified, false)
        } else {
            kind.get_java_name().to_string()
        }
    }

    /// 对应 `toClassName()`：`Class.getName()` 形式。
    fn to_class_name(&self) -> String {
        internal_name_to_java(self.get_name(), true, true)
    }
}
