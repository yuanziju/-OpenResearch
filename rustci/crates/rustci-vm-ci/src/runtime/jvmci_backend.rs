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
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have
 * questions.
 */

//! 镜像 `jdk.vm.ci.runtime.JVMCIBackend`：meta/code/services 提供者聚合。
//!
//! 偏离记录：Java `final class JVMCIBackend`（4 个 `final` 字段 + 构造器 + 5 个 getter）
//! → Rust `pub struct JVMCIBackend` + `impl`。4 字段为 trait 对象 `Box<dyn ...>`（Java 持
//! 接口引用，Rust 持所有权）。`getTarget()` 委托 `codeCache.getTarget()`，返回 owned
//! `TargetDescription`（对齐 `CodeCacheProvider::get_target` 签名）。

use crate::code::code_cache_provider::CodeCacheProvider;
use crate::code::stack::StackIntrospection;
use crate::code::target_description::TargetDescription;
use crate::meta::constant_reflection::ConstantReflectionProvider;
use crate::meta::meta_access::MetaAccessProvider;

/// 对应 `public final class JVMCIBackend`。
pub struct JVMCIBackend {
    meta_access: Box<dyn MetaAccessProvider>,
    code_cache: Box<dyn CodeCacheProvider>,
    constant_reflection: Box<dyn ConstantReflectionProvider>,
    stack_introspection: Box<dyn StackIntrospection>,
}

impl JVMCIBackend {
    /// 对应 `JVMCIBackend(MetaAccessProvider, CodeCacheProvider,
    /// ConstantReflectionProvider, StackIntrospection)`。
    pub fn new(
        meta_access: Box<dyn MetaAccessProvider>,
        code_cache: Box<dyn CodeCacheProvider>,
        constant_reflection: Box<dyn ConstantReflectionProvider>,
        stack_introspection: Box<dyn StackIntrospection>,
    ) -> Self {
        Self {
            meta_access,
            code_cache,
            constant_reflection,
            stack_introspection,
        }
    }

    /// 对应 `MetaAccessProvider getMetaAccess()`。
    pub fn get_meta_access(&self) -> &dyn MetaAccessProvider {
        self.meta_access.as_ref()
    }

    /// 对应 `CodeCacheProvider getCodeCache()`。
    pub fn get_code_cache(&self) -> &dyn CodeCacheProvider {
        self.code_cache.as_ref()
    }

    /// 对应 `ConstantReflectionProvider getConstantReflection()`。
    pub fn get_constant_reflection(&self) -> &dyn ConstantReflectionProvider {
        self.constant_reflection.as_ref()
    }

    /// 对应 `TargetDescription getTarget()`：委托 `codeCache.getTarget()`。
    pub fn get_target(&self) -> TargetDescription {
        self.code_cache.get_target()
    }

    /// 对应 `StackIntrospection getStackIntrospection()`。
    pub fn get_stack_introspection(&self) -> &dyn StackIntrospection {
        self.stack_introspection.as_ref()
    }
}
