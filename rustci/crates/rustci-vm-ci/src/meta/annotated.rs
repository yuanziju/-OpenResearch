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

//! 镜像 `jdk.vm.ci.meta.Annotated`：可承载注解的程序元素（方法/构造/字段/类）。

use crate::meta::annotation_data::AnnotationData;
use crate::meta::resolved_java_type::ResolvedJavaType;

/// 对应 `interface Annotated`。
///
/// 偏离记录：Java 变长参数 `ResolvedJavaType... types` → Rust `&[&dyn ResolvedJavaType]`。
/// Java 默认方法抛 `UnsupportedOperationException` → Rust `panic!`（对齐运行时异常语义）。
pub trait Annotated {
    fn get_annotation_data_many(
        &self,
        _type1: &dyn ResolvedJavaType,
        _type2: &dyn ResolvedJavaType,
        _types: &[&dyn ResolvedJavaType],
    ) -> Vec<AnnotationData> {
        panic!("UnsupportedOperationException");
    }

    fn get_annotation_data(&self, _type_: &dyn ResolvedJavaType) -> Option<AnnotationData> {
        panic!("UnsupportedOperationException");
    }
}
