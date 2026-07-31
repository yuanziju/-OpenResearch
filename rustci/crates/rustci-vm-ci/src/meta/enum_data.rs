// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2023, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.EnumData`：`AnnotationData` 内的枚举常量。
//!
//! 偏离记录：Java `EnumData.equals` 调 `type.equals`，而 `JavaType` 无 `equals` 覆写，
//! 走 `Object.equals`（引用相等，对 `UnresolvedJavaType` 因其覆写 `equals` 而按名相等——
//! 多态分派）。Rust trait 对象无法表达该多态 `Object.equals`，`type` 字段相等性退化为
//! 指针相等；完整多态相等待 `JavaType` 增设 `java_equals` 方法后补齐（T7）。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_type::JavaType;

/// 对应 `final class EnumData`。
pub struct EnumData {
    r#type: Box<dyn JavaType>,
    name: String,
}

impl EnumData {
    /// 对应 `EnumData(JavaType type, String name)`。
    pub fn new(r#type: Box<dyn JavaType>, name: impl Into<String>) -> Self {
        Self {
            r#type,
            name: name.into(),
        }
    }

    /// 对应 `getEnumType()`。
    pub fn get_enum_type(&self) -> &dyn JavaType {
        self.r#type.as_ref()
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Debug for EnumData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumData")
            .field("type", &self.r#type)
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for EnumData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl PartialEq for EnumData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.r#type, &*other.r#type) && self.name == other.name
    }
}

impl Eq for EnumData {}

impl Hash for EnumData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::addr_of!(*self.r#type).hash(state);
        self.name.hash(state);
    }
}
