// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ConstantPool`：编译期字节码解析用的常量池接口。
//!
//! 偏离记录：
//! - Java `lookupConstant(int)` / `lookupConstant(int, boolean)` 返回 `Object`（可为
//!   `Constant` 或 `JavaType`）→ Rust `Option<ConstantPoolEntry>` 判别联合。
//! - Java `lookupMethod(int, int)` 默认方法委托 `lookupMethod(int, int, null)` →
//!   Rust `lookup_method`（默认）/ `lookup_method_with`（带 `caller: Option`）。
//! - Java `loadReferencedType(int, int, boolean)` 默认方法 → Rust
//!   `load_referenced_type_with`（`initialize=true` 调 `load_referenced_type`，`false` panic）。
//! - Java `lookupBootstrapMethodInvocation(int, int)` 默认抛 `UnsupportedOperationException` →
//!   Rust 默认 `panic!`。
//! - Java `lookupAppendix` 返回 nullable → Rust `Option`。
//! - Java `ClassFormatError`/`IllegalAccessError`（非受检）→ Rust `panic!`。

use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_field::JavaField;
use crate::meta::java_method::JavaMethod;
use crate::meta::java_type::JavaType;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::signature::Signature;

/// `lookupConstant` 返回值：对应 Java `Object`（`Constant` 或 `JavaType`）。
#[derive(Debug)]
pub enum ConstantPoolEntry {
    /// 对应 Java 返回 `Constant` 实例（含 `JavaConstant`，因 `JavaConstant extends Constant`）。
    Constant(Box<dyn Constant>),
    /// 对应 Java 返回 `JavaType` 实例。
    JavaType(Box<dyn JavaType>),
}

/// 对应 `ConstantPool.BootstrapMethodInvocation`：bootstrap 方法调用详情。
pub trait BootstrapMethodInvocation {
    /// 对应 `getMethod()`。
    fn get_method(&self) -> &dyn ResolvedJavaMethod;

    /// 对应 `isInvokeDynamic()`。
    fn is_invoke_dynamic(&self) -> bool;

    /// 对应 `getName()`。
    fn get_name(&self) -> &str;

    /// 对应 `getType()`：返回 `JavaConstant`（`MethodType` 或 `Class`）。
    fn get_type(&self) -> &dyn JavaConstant;

    /// 对应 `getStaticArguments()`。
    fn get_static_arguments(&self) -> Vec<Box<dyn JavaConstant>>;

    /// 对应 `resolve()`。
    fn resolve(&self);

    /// 对应 `lookup()`。
    fn lookup(&self) -> Box<dyn JavaConstant>;
}

/// 对应 `public interface ConstantPool`。
pub trait ConstantPool {
    /// 对应 `length()`。
    fn length(&self) -> i32;

    /// 对应 `loadReferencedType(int, int)`。
    fn load_referenced_type(&self, raw_index: i32, opcode: i32);

    /// 对应 `loadReferencedType(int, int, boolean)`（默认方法）。
    fn load_referenced_type_with(&self, raw_index: i32, opcode: i32, initialize: bool) {
        if initialize {
            self.load_referenced_type(raw_index, opcode);
        } else {
            panic!("UnsupportedOperationException");
        }
    }

    /// 对应 `lookupReferencedType(int, int)`。
    fn lookup_referenced_type(&self, raw_index: i32, opcode: i32) -> Box<dyn JavaType>;

    /// 对应 `lookupField(int, ResolvedJavaMethod, int)`。
    fn lookup_field(
        &self,
        raw_index: i32,
        method: &dyn ResolvedJavaMethod,
        opcode: i32,
    ) -> Box<dyn JavaField>;

    /// 对应 `lookupMethod(int, int)`（默认方法，委托 `lookupMethod(int, int, null)`）。
    fn lookup_method(&self, cpi: i32, opcode: i32) -> Box<dyn JavaMethod> {
        self.lookup_method_with(cpi, opcode, None)
    }

    /// 对应 `lookupMethod(int, int, ResolvedJavaMethod)`：`caller` nullable → `Option`。
    fn lookup_method_with(
        &self,
        cpi: i32,
        opcode: i32,
        caller: Option<&dyn ResolvedJavaMethod>,
    ) -> Box<dyn JavaMethod>;

    /// 对应 `lookupBootstrapMethodInvocation(int, int)`（默认抛 `UnsupportedOperationException`）。
    fn lookup_bootstrap_method_invocation(
        &self,
        _index: i32,
        _opcode: i32,
    ) -> Option<Box<dyn BootstrapMethodInvocation>> {
        panic!("UnsupportedOperationException");
    }

    /// 对应 `lookupBootstrapMethodInvocations(boolean)`。
    fn lookup_bootstrap_method_invocations(
        &self,
        invoke_dynamic: bool,
    ) -> Vec<Box<dyn BootstrapMethodInvocation>>;

    /// 对应 `lookupType(int, int)`。
    fn lookup_type(&self, cpi: i32, opcode: i32) -> Box<dyn JavaType>;

    /// 对应 `lookupUtf8(int)`。
    fn lookup_utf8(&self, cpi: i32) -> String;

    /// 对应 `lookupSignature(int)`。
    fn lookup_signature(&self, cpi: i32) -> Box<dyn Signature>;

    /// 对应 `lookupConstant(int)`。
    fn lookup_constant(&self, cpi: i32) -> Option<ConstantPoolEntry>;

    /// 对应 `lookupConstant(int, boolean)`。
    fn lookup_constant_with(&self, cpi: i32, resolve: bool) -> Option<ConstantPoolEntry>;

    /// 对应 `lookupAppendix(int, int)`：返回 `Option` 对齐 Java nullable 语义。
    fn lookup_appendix(&self, raw_index: i32, opcode: i32) -> Option<Box<dyn JavaConstant>>;
}
