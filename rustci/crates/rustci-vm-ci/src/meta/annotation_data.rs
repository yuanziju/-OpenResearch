// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2023, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.AnnotationData`：注解数据（名称 + 元素键值）。
//!
//! 偏离记录：
//! - Java 元素值 `Object` 限定为 `Boolean/Byte/Character/Short/Integer/Float/Long/Double/
//!   String/EnumData/AnnotationData/JavaType/ErrorData/List`，Rust 侧用判别联合
//!   `AnnotationElementValue` 表达全部分支；构造期类型校验（Java 抛
//!   `IllegalArgumentException`）由类型系统承担。
//! - Java `<V> V get(String, Class<V>)` 依赖反射 `Class.cast`，Rust 无等价物；改为
//!   `get(&str) -> &AnnotationElementValue`（缺名/`ErrorData` 时 `panic!`，对齐
//!   `IllegalArgumentException`），调用方按变体下取。
//! - `type` 字段相等性退化为指针相等（同 `EnumData` 偏离）。

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::enum_data::EnumData;
use crate::meta::error_data::ErrorData;
use crate::meta::java_type::JavaType;

/// 注解元素值域，对应 `AnnotationData.get` 文档表中各类型及 `ErrorData`/`List`。
#[derive(Debug)]
pub enum AnnotationElementValue {
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(String),
    Enum(EnumData),
    Annotation(Box<AnnotationData>),
    JavaType(Box<dyn JavaType>),
    Error(ErrorData),
    List(Vec<AnnotationElementValue>),
}

impl PartialEq for AnnotationElementValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Byte(a), Self::Byte(b)) => a == b,
            (Self::Char(a), Self::Char(b)) => a == b,
            (Self::Short(a), Self::Short(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.to_bits() == b.to_bits(),
            (Self::Long(a), Self::Long(b)) => a == b,
            (Self::Double(a), Self::Double(b)) => a.to_bits() == b.to_bits(),
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Enum(a), Self::Enum(b)) => a == b,
            (Self::Annotation(a), Self::Annotation(b)) => a.as_ref() == b.as_ref(),
            (Self::JavaType(a), Self::JavaType(b)) => std::ptr::eq(&**a, &**b),
            (Self::Error(a), Self::Error(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for AnnotationElementValue {}

impl Hash for AnnotationElementValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Boolean(v) => v.hash(state),
            Self::Byte(v) => v.hash(state),
            Self::Char(v) => v.hash(state),
            Self::Short(v) => v.hash(state),
            Self::Int(v) => v.hash(state),
            Self::Float(v) => v.to_bits().hash(state),
            Self::Long(v) => v.hash(state),
            Self::Double(v) => v.to_bits().hash(state),
            Self::String(v) => v.hash(state),
            Self::Enum(v) => v.hash(state),
            Self::Annotation(v) => v.hash(state),
            Self::JavaType(v) => {
                // 对齐指针地址哈希：先绑定引用避免取临时地址。
                let r: &dyn JavaType = &**v;
                let ptr: *const dyn JavaType = r;
                ptr.hash(state);
            }
            Self::Error(v) => v.hash(state),
            Self::List(v) => v.hash(state),
        }
    }
}

/// 对应 `final class AnnotationData`。
pub struct AnnotationData {
    r#type: Box<dyn JavaType>,
    elements: BTreeMap<String, AnnotationElementValue>,
}

impl AnnotationData {
    /// 对应 `AnnotationData(JavaType type, Map.Entry<String, Object>[] elements)`。
    pub fn new(r#type: Box<dyn JavaType>, elements: Vec<(String, AnnotationElementValue)>) -> Self {
        let map = elements.into_iter().collect::<BTreeMap<String, _>>();
        Self {
            r#type,
            elements: map,
        }
    }

    /// 对应 `getAnnotationType()`。
    pub fn get_annotation_type(&self) -> &dyn JavaType {
        self.r#type.as_ref()
    }

    /// 对应 `<V> V get(String name, Class<V> elementType)`：缺名或 `ErrorData` 时
    /// `panic!`（对齐 `IllegalArgumentException`）；返回元素值由调用方按变体下取。
    pub fn get(&self, name: &str) -> &AnnotationElementValue {
        match self.elements.get(name) {
            None => panic!("no element named {}", name),
            Some(AnnotationElementValue::Error(e)) => panic!("{}", e),
            Some(v) => v,
        }
    }

    /// 元素表（只读视图）。
    pub fn elements(&self) -> &BTreeMap<String, AnnotationElementValue> {
        &self.elements
    }
}

impl fmt::Debug for AnnotationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnnotationData")
            .field("type", &self.r#type)
            .field("elements", &self.elements)
            .finish()
    }
}

impl fmt::Display for AnnotationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}({:?})", self.r#type.get_name(), self.elements)
    }
}

impl PartialEq for AnnotationData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.r#type, &*other.r#type) && self.elements == other.elements
    }
}

impl Eq for AnnotationData {}

impl Hash for AnnotationData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::addr_of!(*self.r#type).hash(state);
        self.elements.hash(state);
    }
}
