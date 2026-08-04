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

//! 镜像 `jdk.vm.ci.code.RegisterValue`：存储固定种类值的寄存器。
//!
//! 偏离记录：Java `final class RegisterValue extends AllocatableValue`
//! → Rust `pub struct RegisterValue` 持 `kind: Box<dyn ValueKind>` + `reg: Register`，
//! 实现 `Value` + `AllocatableValue` + `JavaValue`（组合替代继承，对齐 `meta::value` 偏离）。
//! `hashCode`（`29 * super.hashCode() + reg.hashCode()`）→ `value_hash` 覆写。
//! `equals`（`super.equals(obj) && reg.equals(other.reg)`）→ `value_equals` 覆写。

use std::any::Any;
use std::fmt;

use crate::code::register::Register;
use crate::meta::allocatable_value::AllocatableValue;
use crate::meta::java_value::JavaValue;
use crate::meta::value::Value;
use crate::meta::value_kind::ValueKind;

/// 对应 `final class RegisterValue extends AllocatableValue`。
pub struct RegisterValue {
    kind: Box<dyn ValueKind>,
    reg: Register,
}

impl RegisterValue {
    /// 对应 `protected RegisterValue(ValueKind<?> kind, Register register)`。
    pub fn new(kind: Box<dyn ValueKind>, register: Register) -> Self {
        Self {
            kind,
            reg: register,
        }
    }

    /// 对应 `getRegister()`。
    pub fn get_register(&self) -> Register {
        self.reg
    }
}

impl fmt::Debug for RegisterValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegisterValue")
            .field("kind", &self.kind)
            .field("reg", &self.reg)
            .finish()
    }
}

impl fmt::Display for RegisterValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString() { return getRegister().name + getKindSuffix(); }`
        write!(f, "{}{}", self.reg.name, self.get_kind_suffix())
    }
}

impl JavaValue for RegisterValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Value for RegisterValue {
    fn get_value_kind(&self) -> &dyn ValueKind {
        self.kind.as_ref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Value> {
        Box::new(RegisterValue {
            kind: self.kind.clone_box(),
            reg: self.reg,
        })
    }

    fn value_equals(&self, other: &dyn Value) -> bool {
        // super.equals（按 valueKind.equals）+ reg.equals。
        match other.as_any().downcast_ref::<RegisterValue>() {
            None => false,
            Some(o) => self.kind.kind_equals(o.kind.as_ref()) && self.reg == o.reg,
        }
    }

    fn value_hash(&self) -> u64 {
        // super.hashCode() = 41 + valueKind.hashCode()（对齐 `Value.hashCode` 默认实现）。
        let super_h = 41u64.wrapping_add(self.kind.kind_hash());
        // 29 * super.hashCode() + reg.hashCode()。
        29u64
            .wrapping_mul(super_h)
            .wrapping_add(self.reg.number as u64)
    }
}

impl AllocatableValue for RegisterValue {}
