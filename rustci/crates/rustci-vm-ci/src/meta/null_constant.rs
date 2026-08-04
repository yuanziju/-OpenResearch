// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2015, Oracle and/or its affiliates. All rights reserved.
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
 * or visit www.oracle if you need additional information or have any
 * questions.
 */

//! 镜像 `jdk.vm.ci.meta.NullConstant`：`JavaConstant.NULL_POINTER` 的实现类型。
//!
//! 偏离记录：Java `final class NullConstant implements JavaConstant` → Rust `struct NullConstant`
//! 实现 `JavaConstant` trait。`asBoxedPrimitive()`/`asInt()`/`asBoolean()`/`asLong()`/`asFloat()`/
//! `asDouble()` 抛 `IllegalArgumentException` → Rust `panic!`。`hashCode` 返回 13。

use std::fmt;

use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_kind::{JavaKind, JavaObjectValue};
use crate::meta::java_value::JavaValue;

/// 对应 `final class NullConstant implements JavaConstant`。
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NullConstant;

impl NullConstant {
    /// 对应 `protected NullConstant()`。
    pub fn new() -> Self {
        NullConstant
    }
}

impl Default for NullConstant {
    fn default() -> Self {
        Self::new()
    }
}

impl JavaConstant for NullConstant {
    fn get_java_kind(&self) -> JavaKind {
        JavaKind::Object
    }

    fn is_null(&self) -> bool {
        true
    }

    fn as_boxed_primitive(&self) -> JavaObjectValue {
        panic!("IllegalArgumentException");
    }

    fn as_int(&self) -> i32 {
        panic!("IllegalArgumentException");
    }

    fn as_boolean(&self) -> bool {
        panic!("IllegalArgumentException");
    }

    fn as_long(&self) -> i64 {
        panic!("IllegalArgumentException");
    }

    fn as_float(&self) -> f32 {
        panic!("IllegalArgumentException");
    }

    fn as_double(&self) -> f64 {
        panic!("IllegalArgumentException");
    }

    fn clone_box(&self) -> Box<dyn JavaConstant> {
        Box::new(NullConstant)
    }

    fn constant_equals(&self, other: &dyn JavaConstant) -> bool {
        // 对应 `NullConstant.equals(Object o) { return o instanceof NullConstant; }`
        Constant::as_any(other)
            .downcast_ref::<NullConstant>()
            .is_some()
    }
}

impl Constant for NullConstant {
    fn is_default_for_kind(&self) -> bool {
        true
    }

    fn to_value_string(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl JavaValue for NullConstant {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for NullConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `NullConstant.toString() { return JavaConstant.toString(this); }`
        f.write_str(&crate::meta::java_constant::to_string(self))
    }
}
