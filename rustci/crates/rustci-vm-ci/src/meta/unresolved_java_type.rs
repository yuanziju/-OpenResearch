// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES IN THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details.
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.UnresolvedJavaType`：未解析类型的占位实现。
//!
//! 偏离记录：
//! - Java `final class UnresolvedJavaType implements JavaType` → Rust `pub struct UnresolvedJavaType`
//!   实现 `JavaType` trait。
//! - Java `Throwable cause` 字段（可空）→ Rust `Option<Box<dyn Any + Send + Sync>>`（对齐
//!   Java `Throwable` 的不透明语义；T7 绑定具体错误类型时替换）。
//! - Java `getComponentType()` 每次调用 `new UnresolvedJavaType(name.substring(1), null)` 新建
//!   对象；Rust 侧 `JavaType::get_component_type` 返回引用 `Option<&dyn JavaType>`，无法返回
//!   新建临时值，故构造期预计算并存储组件类型（数组维度链浅，行为等价；返回引用指向同一存储）。
//! - Java `resolve(ResolvedJavaType)` 调 `accessingClass.lookupType(this, true)`，默认实现抛
//!   `UnsupportedOperationException`；Rust 侧 `lookup_type` 默认返回 `None`，故 `resolve` 在
//!   `None` 时 `panic!`（对齐 Java 抛异常语义）。
//! - Java 构造期 `assert` 校验 name 形态；Java `fromPrimitiveOrVoidTypeChar` 返回 nullable，
//!   Rust 侧同名函数 panic，故断言改为内联字符匹配（`Z/C/F/D/B/S/I/J/V` 之一）。
//! - Java `equals` 按 name 字符串相等；Rust 侧 `PartialEq` 同。
//! - Java `hashCode` 返回 `getName().hashCode()`；Rust 侧 `Hash` 写 name 字节（对齐 str 哈希）。

use std::fmt;

use crate::meta::java_kind::JavaKind;
use crate::meta::java_type::JavaType;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `public final class UnresolvedJavaType implements JavaType`。
pub struct UnresolvedJavaType {
    name: String,
    /// 对应 Java `Throwable cause`（可空）。
    cause: Option<Box<dyn std::any::Any + Send + Sync>>,
    /// 预计算的组件类型（数组时 `Some`，非数组 `None`）。
    component_type: Option<Box<UnresolvedJavaType>>,
}

impl UnresolvedJavaType {
    /// 对应私有构造器 `UnresolvedJavaType(String name, Throwable cause)`。
    fn new(name: String, cause: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // 对齐 Java 断言：单字符 primitive/void 类型名，或数组前缀 `[`，或对象后缀 `;`。
        let first = name.chars().next();
        let last = name.chars().last();
        let is_single_primitive = name.chars().count() == 1
            && matches!(
                first,
                Some('Z')
                    | Some('C')
                    | Some('F')
                    | Some('D')
                    | Some('B')
                    | Some('S')
                    | Some('I')
                    | Some('J')
                    | Some('V')
            );
        let valid = is_single_primitive || first == Some('[') || last == Some(';');
        debug_assert!(valid, "{}", name);
        // 预计算组件类型（对齐 Java getComponentType 的 lazy 新建语义，Rust 侧提前存储）。
        let component_type = if first == Some('[') {
            // 对应 `getName().substring(1)`：去掉首字符 `[`，cause 为 null。
            let sub = name[1..].to_string();
            Some(Box::new(UnresolvedJavaType::new(sub, None)))
        } else {
            None
        };
        Self {
            name,
            cause,
            component_type,
        }
    }

    /// 对应 `static UnresolvedJavaType create(String name)`。
    pub fn create(name: &str) -> Self {
        Self::new(name.to_string(), None)
    }

    /// 对应 `static UnresolvedJavaType create(String name, Throwable cause)`。
    pub fn create_with_cause(name: &str, cause: Box<dyn std::any::Any + Send + Sync>) -> Self {
        Self::new(name.to_string(), Some(cause))
    }

    /// 对应 `Throwable getCause()`：返回 `Option` 对齐 Java nullable。
    pub fn get_cause(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
        self.cause.as_deref()
    }
}

impl JavaType for UnresolvedJavaType {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_component_type(&self) -> Option<&dyn JavaType> {
        // 返回预计算组件类型的引用（对齐 Java nullable：非数组返回 None）。
        self.component_type.as_deref().map(|t| t as &dyn JavaType)
    }

    fn get_array_class(&self) -> Box<dyn JavaType> {
        // 对应 `'[' + getName()`。
        Box::new(UnresolvedJavaType::new(format!("[{}", self.name), None))
    }

    fn get_java_kind(&self) -> JavaKind {
        JavaKind::Object
    }

    fn resolve(&self, accessing_class: &dyn ResolvedJavaType) -> Box<dyn ResolvedJavaType> {
        // 对应 `accessingClass.lookupType(this, true)`；默认实现返回 None → panic 对齐 Java 抛异常。
        accessing_class
            .lookup_type(self, true)
            .unwrap_or_else(|| panic!("lookupType returned null for {}", self.name))
    }
}

impl PartialEq for UnresolvedJavaType {
    fn eq(&self, other: &Self) -> bool {
        // 对应 Java `equals`：name 字符串相等（`this == obj` 由 Rust 引用相等覆盖）。
        self.name == other.name
    }
}

impl Eq for UnresolvedJavaType {}

impl std::hash::Hash for UnresolvedJavaType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // 对应 `hashCode() { return getName().hashCode(); }`。
        self.name.hash(state);
    }
}

impl fmt::Debug for UnresolvedJavaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对齐 `toString()` 输出。
        write!(f, "UnresolvedJavaType<{}>", self.name)
    }
}

impl fmt::Display for UnresolvedJavaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnresolvedJavaType<{}>", self.name)
    }
}
