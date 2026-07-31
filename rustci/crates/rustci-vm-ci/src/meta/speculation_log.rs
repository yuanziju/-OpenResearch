// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2012, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.SpeculationLog`：编译期推测（speculation）的管理接口。
//!
//! 偏离记录：
//! - Java 嵌套 `interface SpeculationReason` → Rust 同文件 `pub trait SpeculationReason`。
//!   Java `encode(Supplier<SpeculationReasonEncoding>)` 默认返回 `null` → Rust 默认返回 `None`。
//!   `Supplier` 参数改为 `Box<dyn FnOnce() -> Option<Box<dyn SpeculationReasonEncoding>>>`
//!   以对齐 `Supplier<SpeculationReasonEncoding>` 语义（供应者返回 nullable encoding）。
//! - Java 嵌套 `interface SpeculationReasonEncoding` → Rust `pub trait SpeculationReasonEncoding`。
//!   Java `addField(ResolvedJavaField)` 默认方法委托 `addType`/`addInt` → Rust 默认方法一致。
//! - Java `final class NoSpeculationReason implements SpeculationReason` → Rust `pub struct NoSpeculationReason`。
//! - Java `class Speculation`（含 `reason` 字段 + `equals`/`hashCode`/`toString`）→ Rust `pub struct Speculation`。
//!   `equals` 基于 `reason.equals` → Rust 侧 `SpeculationReason` 为 trait 对象，增设 `reason_equals`
//!   方法由实现者提供值相等语义（对齐 Java `SpeculationReason.equals`/`hashCode` 契约）。
//! - Java `SpeculationLog.NO_SPECULATION` 静态字段 → Rust `fn no_speculation() -> Speculation`
//!   （每次返回等值新对象，`NoSpeculationReason` 无字段，值相等一致）。
//! - Java `speculate(SpeculationReason)` 返回 `Speculation` → Rust 返回 `Speculation`（值类型）。
//! - Java `lookupSpeculation(JavaConstant)` 返回 `Speculation` → Rust 返回 `Speculation`。

use std::hash::{Hash, Hasher};

use crate::meta::java_constant::JavaConstant;
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `SpeculationLog.SpeculationReason`：推测的特定属性集合。
///
/// 偏离记录：Java `SpeculationReason` 作为 `Map` key 需 `equals`/`hashCode`；
/// Rust 侧增设 `reason_equals`/`reason_hash` 由实现者提供，对齐 Java 契约。增设 `Debug`
/// 超 trait：`Speculation` 持 `Box<dyn SpeculationReason>` 需 `Debug` 派生。
pub trait SpeculationReason: std::fmt::Debug {
    /// 对应 `encode(Supplier<SpeculationReasonEncoding>)`（默认返回 `None`，对齐 Java `null`）。
    fn encode(
        &self,
        encoding_supplier: Box<dyn FnOnce() -> Option<Box<dyn SpeculationReasonEncoding>>>,
    ) -> Option<Box<dyn SpeculationReasonEncoding>> {
        let _ = encoding_supplier;
        None
    }

    /// Rust 增设：对应 Java `Object.equals`（`SpeculationReason` 实现者覆写）。
    fn reason_equals(&self, other: &dyn SpeculationReason) -> bool;

    /// Rust 增设：对应 Java `Object.hashCode`（`SpeculationReason` 实现者覆写）。
    fn reason_hash(&self, state: &mut dyn Hasher);

    /// Rust 增设：支持 trait 对象下转（`NoSpeculationReason` 判定）。
    fn as_any(&self) -> &dyn std::any::Any;
}

/// 对应 `SpeculationLog.SpeculationReasonEncoding`：`SpeculationReason` 属性编码设施。
pub trait SpeculationReasonEncoding {
    /// 对应 `addByte(int)`。
    fn add_byte(&mut self, value: i32);
    /// 对应 `addShort(int)`。
    fn add_short(&mut self, value: i32);
    /// 对应 `addInt(int)`。
    fn add_int(&mut self, value: i32);
    /// 对应 `addLong(long)`。
    fn add_long(&mut self, value: i64);
    /// 对应 `addMethod(ResolvedJavaMethod)`。
    fn add_method(&mut self, method: &dyn ResolvedJavaMethod);
    /// 对应 `addType(ResolvedJavaType)`。
    fn add_type(&mut self, type_: &dyn ResolvedJavaType);
    /// 对应 `addString(String)`。
    fn add_string(&mut self, value: &str);

    /// 对应 `addField(ResolvedJavaField)`（默认方法）。
    ///
    /// 偏离记录：Java `field.getDeclaringClass()` 协变返回 `ResolvedJavaType`；Rust 侧
    /// `JavaField::get_declaring_class` 返回 `&dyn JavaType`，故改用 `ResolvedJavaField::
    /// get_resolved_declaring_class`（Rust 增设）取 `&dyn ResolvedJavaType`。
    fn add_field(&mut self, field: &dyn ResolvedJavaField) {
        self.add_type(field.get_resolved_declaring_class());
        self.add_int(field.get_modifiers());
        self.add_int(field.get_offset());
    }
}

/// 对应 `SpeculationLog.NoSpeculationReason`：标记无推测原因的 marker class。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct NoSpeculationReason;

impl SpeculationReason for NoSpeculationReason {
    fn reason_equals(&self, other: &dyn SpeculationReason) -> bool {
        other.as_any().is::<NoSpeculationReason>()
    }

    fn reason_hash(&self, state: &mut dyn Hasher) {
        // 对齐 `"NoSpeculationReason".hash(state)`：str 的 Hash 写字节后接 `0u8` 终止符。
        state.write(b"NoSpeculationReason");
        state.write_u8(0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// 对应 `SpeculationLog.Speculation`：一次推测的封装。
#[derive(Debug)]
pub struct Speculation {
    reason: Box<dyn SpeculationReason>,
}

impl Speculation {
    /// 对应 `Speculation(SpeculationReason reason)`。
    pub fn new(reason: Box<dyn SpeculationReason>) -> Self {
        Self { reason }
    }

    /// 对应 `getReason()`。
    pub fn get_reason(&self) -> &dyn SpeculationReason {
        self.reason.as_ref()
    }

    /// 对应 `Speculation.equals`：基于 `reason.equals`。
    pub fn speculation_equals(&self, other: &Speculation) -> bool {
        self.reason.reason_equals(other.reason.as_ref())
    }

    /// 对应 `Speculation.hashCode`：基于 `reason.hashCode`。
    pub fn speculation_hash<H: Hasher>(&self, state: &mut H) {
        self.reason.reason_hash(state);
    }
}

impl Clone for Speculation {
    fn clone(&self) -> Self {
        // NoSpeculationReason 无字段，可安全克隆；其它实现者 T7 提供共享指针后扩展。
        if self.reason.as_any().is::<NoSpeculationReason>() {
            return Self::new(Box::new(NoSpeculationReason));
        }
        panic!(
            "Speculation::clone requires shared-pointer support for non-NoSpeculationReason (T7)"
        );
    }
}

/// 对应 `SpeculationLog.NO_SPECULATION` 静态字段。
pub fn no_speculation() -> Speculation {
    Speculation::new(Box::new(NoSpeculationReason))
}

/// 对应 `public interface SpeculationLog`。
pub trait SpeculationLog {
    /// 对应 `collectFailedSpeculations()`。
    fn collect_failed_speculations(&self);

    /// 对应 `maySpeculate(SpeculationReason)`。
    fn may_speculate(&self, reason: &dyn SpeculationReason) -> bool;

    /// 对应 `speculate(SpeculationReason)`：返回 `Speculation`。
    fn speculate(&self, reason: &dyn SpeculationReason) -> Speculation;

    /// 对应 `hasSpeculations()`。
    fn has_speculations(&self) -> bool;

    /// 对应 `lookupSpeculation(JavaConstant)`。
    fn lookup_speculation(&self, constant: &dyn JavaConstant) -> Speculation;
}
