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

//! 镜像 `jdk.graal.compiler.nodes.ReturnNode`：方法返回节点。
//!
//! 偏离记录：Java `ReturnNode` 继承 `MemoryMapControlSinkNode` 并实现 `LIRLowerable`。
//! Rust 侧为具体 struct，保留核心字段。

use rustci_vm_ci::meta::java_kind::JavaKind;

use crate::nodes::control_sink_node::ControlSinkNode;
use crate::nodes::fixed_node::FixedNode;
use crate::nodes::value_node::ValueNode;

/// 对应 `final class ReturnNode extends MemoryMapControlSinkNode implements LIRLowerable`。
///
/// 方法返回节点。控制流在此结束。
#[derive(Debug)]
pub struct ReturnNode {
    /// 对应 `result`：返回值。None 表示 void return。
    pub result: Option<Box<dyn ValueNode>>,
}

impl ReturnNode {
    /// 创建一个新的 ReturnNode。
    pub fn new(result: Option<Box<dyn ValueNode>>) -> Self {
        ReturnNode { result }
    }

    /// 对应 `result()`：获取返回值。
    pub fn result(&self) -> Option<&dyn ValueNode> {
        self.result.as_ref().map(|r| r.as_ref())
    }
}

impl ValueNode for ReturnNode {
    fn get_stack_kind(&self) -> JavaKind {
        JavaKind::Void
    }
}

impl FixedNode for ReturnNode {}

impl ControlSinkNode for ReturnNode {}
