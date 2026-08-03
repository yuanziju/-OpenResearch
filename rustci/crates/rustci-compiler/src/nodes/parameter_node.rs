// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
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

//! 镜像 `jdk.graal.compiler.nodes.ParameterNode`：方法参数节点。
//!
//! 偏离记录：Java `ParameterNode` 继承 `AbstractLocalNode` 并实现 `IterableNodeType`、
//! `UncheckedInterfaceProvider`。Rust 侧为具体 struct。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::value_node::ValueNode;

/// 对应 `final class ParameterNode extends AbstractLocalNode implements IterableNodeType, UncheckedInterfaceProvider`。
///
/// 表示函数调用的传入参数占位符。
#[derive(Debug, Clone)]
pub struct ParameterNode {
    /// 对应 `index()`：参数索引。
    pub index: i32,
    /// 对应 `stamp` 的 trusted stamp：参数的种类。
    pub kind: JavaKind,
    /// 对应 `uncheckedStamp`：未检查的 stamp（用于接口类型转换前）。
    pub unchecked_kind: JavaKind,
}

impl ParameterNode {
    /// 创建一个新的 ParameterNode。
    pub fn new(index: i32, kind: JavaKind, unchecked_kind: JavaKind) -> Self {
        ParameterNode {
            index,
            kind,
            unchecked_kind,
        }
    }

    /// 对应 `index()`：获取参数索引。
    pub fn index(&self) -> i32 {
        self.index
    }

    /// 对应 `uncheckedStamp()`：获取未检查的 stamp。
    pub fn unchecked_kind(&self) -> JavaKind {
        self.unchecked_kind
    }
}

impl ValueNode for ParameterNode {
    fn get_stack_kind(&self) -> JavaKind {
        self.kind.get_stack_kind()
    }
}
