// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.UnresolvedJavaField`：未解析字段的占位实现。
//!
//! 偏离记录：
//! - Java `final class UnresolvedJavaField implements JavaField` → Rust `pub struct
//!   UnresolvedJavaField` 实现 `JavaField` trait。
//! - Java `Throwable cause`（可空）→ Rust `Option<Box<dyn Any + Send + Sync>>`（对齐
//!   Java `Throwable` 的不透明语义；T7 绑定具体错误类型时替换，同 `UnresolvedJavaType`）。
//! - Java `equals` 调 `holder.equals`/`type.equals`；`JavaType` 无多态 `equals` 覆写
//!   （HotSpot `ResolvedJavaType.equals` 引用相等，`UnresolvedJavaType.equals` 名相等）。
//!   Rust trait 对象无法表达该多态 `Object.equals`，`holder`/`type` 字段相等性退化为
//!   指针相等（同 `Local`/`ExceptionHandler` 偏离）。
//! - Java `hashCode` 返回 `super.hashCode()`（identity）→ Rust 用指针地址哈希占位
//!   （同 `Local` 偏离；T7 用共享指针地址）。
//! - Java `resolve` 调 `holder.resolve(accessingClass).resolveField(this, accessingClass)`，
//!   `resolveField` 默认返回 `null` → Rust 侧 `resolve_field` 默认返回 `None`，故 `resolve`
//!   在 `None` 时 `panic!`（对齐 Java 抛 `NullPointerException` 语义）。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_field::JavaField;
use crate::meta::java_type::JavaType;
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `public final class UnresolvedJavaField implements JavaField`。
pub struct UnresolvedJavaField {
    name: String,
    holder: Box<dyn JavaType>,
    r#type: Box<dyn JavaType>,
    /// 对应 Java `Throwable cause`（可空）。
    cause: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl UnresolvedJavaField {
    /// 对应 `UnresolvedJavaField(JavaType holder, String name, JavaType type, Throwable cause)`。
    pub fn new(
        holder: Box<dyn JavaType>,
        name: impl Into<String>,
        r#type: Box<dyn JavaType>,
        cause: Option<Box<dyn std::any::Any + Send + Sync>>,
    ) -> Self {
        Self {
            name: name.into(),
            holder,
            r#type,
            cause,
        }
    }

    /// 对应 `UnresolvedJavaField(JavaType holder, String name, JavaType type)`：cause 为 null。
    pub fn without_cause(
        holder: Box<dyn JavaType>,
        name: impl Into<String>,
        r#type: Box<dyn JavaType>,
    ) -> Self {
        Self::new(holder, name, r#type, None)
    }

    /// 对应 `Throwable getCause()`：返回 `Option` 对齐 Java nullable。
    pub fn get_cause(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
        self.cause.as_deref()
    }

    /// 对应 `ResolvedJavaField resolve(ResolvedJavaType accessingClass)`。
    pub fn resolve(&self, accessing_class: &dyn ResolvedJavaType) -> Box<dyn ResolvedJavaField> {
        // 对应 `holder.resolve(accessingClass).resolveField(this, accessingClass)`。
        let resolved_holder = self.holder.resolve(accessing_class);
        resolved_holder
            .resolve_field(self, accessing_class)
            .unwrap_or_else(|| panic!("resolveField returned null for {}", self.name))
    }
}

impl JavaField for UnresolvedJavaField {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> &dyn JavaType {
        self.r#type.as_ref()
    }

    fn get_declaring_class(&self) -> &dyn JavaType {
        self.holder.as_ref()
    }
}

impl PartialEq for UnresolvedJavaField {
    fn eq(&self, other: &Self) -> bool {
        // 对应 Java `equals`：name 字符串相等 + holder/type 引用相等（偏离：多态 equals 退化）。
        self.name == other.name
            && std::ptr::eq(&*self.holder, &*other.holder)
            && std::ptr::eq(&*self.r#type, &*other.r#type)
    }
}

impl Eq for UnresolvedJavaField {}

impl Hash for UnresolvedJavaField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 对应 `super.hashCode()`（identity hash）；Rust 用指针地址占位。
        let addr = self as *const Self as usize;
        (addr as u64).hash(state);
    }
}

impl fmt::Debug for UnresolvedJavaField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对齐 `toString()` 输出（`format("UnresolvedJavaField<%H.%n %t>")`）。
        f.write_str(&JavaField::format(self, "UnresolvedJavaField<%H.%n %t>"))
    }
}

impl fmt::Display for UnresolvedJavaField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`：`format("UnresolvedJavaField<%H.%n %t>")`。
        f.write_str(&JavaField::format(self, "UnresolvedJavaField<%H.%n %t>"))
    }
}
