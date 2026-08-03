// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2015, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.Signature`：方法签名（参数/返回类型）。
//!
//! 偏离记录：`accessingClass` nullable → `Option<&dyn ResolvedJavaType>`；
//! `getParameterType`/`getReturnType` 返回产生值 → `Box<dyn JavaType>`。

use crate::meta::java_kind::JavaKind;
use crate::meta::java_type::JavaType;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface Signature`。
pub trait Signature {
    /// 对应 `getParameterCount(boolean receiver)`。
    fn get_parameter_count(&self, receiver: bool) -> i32;

    /// 对应 `getParameterType(int index, ResolvedJavaType accessingClass)`。
    fn get_parameter_type(
        &self,
        index: i32,
        accessing_class: Option<&dyn ResolvedJavaType>,
    ) -> Box<dyn JavaType>;

    /// 对应 `getParameterKind(int index)`。
    fn get_parameter_kind(&self, index: i32) -> JavaKind {
        self.get_parameter_type(index, None).get_java_kind()
    }

    /// 对应 `getReturnType(ResolvedJavaType accessingClass)`。
    fn get_return_type(&self, accessing_class: Option<&dyn ResolvedJavaType>) -> Box<dyn JavaType>;

    /// 对应 `getReturnKind()`。
    fn get_return_kind(&self) -> JavaKind {
        self.get_return_type(None).get_java_kind()
    }

    /// 对应 `toMethodDescriptor()`。
    fn to_method_descriptor(&self) -> String {
        let mut sb = String::from("(");
        for i in 0..self.get_parameter_count(false) {
            sb.push_str(self.get_parameter_type(i, None).get_name());
        }
        sb.push(')');
        sb.push_str(self.get_return_type(None).get_name());
        sb
    }

    /// 对应 `toParameterTypes(JavaType receiverType)`。
    fn to_parameter_types(
        &self,
        receiver_type: Option<Box<dyn JavaType>>,
    ) -> Vec<Box<dyn JavaType>> {
        let args = self.get_parameter_count(false);
        let mut result: Vec<Box<dyn JavaType>> = Vec::with_capacity(args as usize + 1);
        if let Some(r) = receiver_type {
            result.push(r);
        }
        for j in 0..args {
            result.push(self.get_parameter_type(j, None));
        }
        result
    }

    /// 对应 `toParameterKinds(boolean receiver)`。
    fn to_parameter_kinds(&self, receiver: bool) -> Vec<JavaKind> {
        let args = self.get_parameter_count(false);
        let mut result: Vec<JavaKind> = Vec::with_capacity(args as usize + 1);
        if receiver {
            result.push(JavaKind::Object);
        }
        for j in 0..args {
            result.push(self.get_parameter_kind(j));
        }
        result
    }
}
