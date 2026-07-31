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

//! 镜像 `jdk.vm.ci.meta.MetaAccessProvider`：类文件元数据访问接口。
//!
//! 偏离记录：
//! - Java `lookupJavaType(Class<?>)` / `lookupJavaTypes(Class<?>[])` 依赖 Java 反射 `Class<?>`
//!   → Rust `&JavaClass`（不透明标记，T7 绑定真实 JVM 反射对象）。
//! - Java `lookupJavaMethod(Executable)` / `lookupJavaField(Field)` 依赖 Java 反射
//!   `Executable`/`Field` → Rust `&dyn JavaExecutable`/`&dyn JavaReflectField`（不透明标记）。
//! - Java `lookupJavaType(JavaConstant)` 返回 nullable `ResolvedJavaType` → Rust `Option`。
//! - Java `parseMethodDescriptor(String)` 抛 `IllegalArgumentException`（非受检）→ Rust `panic!`。
//! - Java `decodeSpeculation(JavaConstant, SpeculationLog)` 抛 `IllegalArgumentException`
//!   （非受控）→ 实现侧 `panic!` 或返回值（签名不体现受检异常）。
//! - Java `lookupJavaTypes(Class<?>[])` 默认方法逐元素调 `lookupJavaType` → Rust 默认方法一致。

use crate::meta::deoptimization::{DeoptimizationAction, DeoptimizationReason};
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_reflect::{JavaClass, JavaExecutable, JavaReflectField};
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::resolved_java_type::ResolvedJavaType;
use crate::meta::signature::Signature;
use crate::meta::speculation_log::{Speculation, SpeculationLog};

/// 对应 `public interface MetaAccessProvider`。
pub trait MetaAccessProvider {
    /// 对应 `lookupJavaType(Class<?>)`。
    fn lookup_java_type(&self, klass: &JavaClass) -> Box<dyn ResolvedJavaType>;

    /// 对应 `lookupJavaTypes(Class<?>[])`（默认方法）。
    fn lookup_java_types(&self, classes: &[&JavaClass]) -> Vec<Box<dyn ResolvedJavaType>> {
        let mut result = Vec::with_capacity(classes.len());
        for c in classes {
            result.push(self.lookup_java_type(c));
        }
        result
    }

    /// 对应 `lookupJavaMethod(Executable)`。
    fn lookup_java_method(
        &self,
        reflection_method: &dyn JavaExecutable,
    ) -> Box<dyn ResolvedJavaMethod>;

    /// 对应 `lookupJavaField(Field)`。
    fn lookup_java_field(
        &self,
        reflection_field: &dyn JavaReflectField,
    ) -> Box<dyn ResolvedJavaField>;

    /// 对应 `lookupJavaType(JavaConstant)`：返回 `Option` 对齐 Java nullable。
    fn lookup_java_type_from_constant(
        &self,
        constant: &dyn JavaConstant,
    ) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `getMemorySize(JavaConstant)`。
    fn get_memory_size(&self, constant: &dyn JavaConstant) -> i64;

    /// 对应 `parseMethodDescriptor(String)`。
    fn parse_method_descriptor(&self, method_descriptor: &str) -> Box<dyn Signature>;

    /// 对应 `encodeDeoptActionAndReason(DeoptimizationAction, DeoptimizationReason, int)`。
    fn encode_deopt_action_and_reason(
        &self,
        action: DeoptimizationAction,
        reason: DeoptimizationReason,
        debug_id: i32,
    ) -> Box<dyn JavaConstant>;

    /// 对应 `encodeSpeculation(Speculation)`。
    fn encode_speculation(&self, speculation: &Speculation) -> Box<dyn JavaConstant>;

    /// 对应 `decodeSpeculation(JavaConstant, SpeculationLog)`。
    fn decode_speculation(
        &self,
        constant: &dyn JavaConstant,
        speculation_log: &dyn SpeculationLog,
    ) -> Speculation;

    /// 对应 `decodeDeoptReason(JavaConstant)`。
    fn decode_deopt_reason(&self, constant: &dyn JavaConstant) -> DeoptimizationReason;

    /// 对应 `decodeDeoptAction(JavaConstant)`。
    fn decode_deopt_action(&self, constant: &dyn JavaConstant) -> DeoptimizationAction;

    /// 对应 `decodeDebugId(JavaConstant)`。
    fn decode_debug_id(&self, constant: &dyn JavaConstant) -> i32;

    /// 对应 `getArrayBaseOffset(JavaKind)`。
    fn get_array_base_offset(&self, element_kind: JavaKind) -> i32;

    /// 对应 `getArrayIndexScale(JavaKind)`。
    fn get_array_index_scale(&self, element_kind: JavaKind) -> i32;
}
