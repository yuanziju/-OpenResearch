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

//! 镜像 `jdk.graal.compiler.nodes.extended.GuardingNode`：守卫节点接口。
//!
//! 偏离记录：Java `GuardingNode` 在 `extended` 包中，继承 `ValueNodeInterface`。
//! Rust 侧为 trait，继承 `ValueNode`。

use crate::nodes::value_node::ValueNode;

/// 对应 `interface GuardingNode extends ValueNodeInterface`。
///
/// 守卫节点：用于在 IR 中编码守卫条件（例如 PiNode 的锚点）。
/// GuardingNode 可以用作数据依赖的锚点，防止被守卫的节点浮动到守卫之前。
pub trait GuardingNode: ValueNode {
    /// 返回此守卫节点作为 ValueNode 引用。
    fn as_node(&self) -> &dyn ValueNode
    where
        Self: Sized,
    {
        self
    }
}
