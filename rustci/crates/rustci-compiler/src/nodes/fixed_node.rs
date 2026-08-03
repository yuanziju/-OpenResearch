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

//! 镜像 `jdk.graal.compiler.nodes.FixedNode`：控制流图中位置固定的节点抽象基类。
//!
//! 偏离记录：Java `FixedNode` 继承 `ValueNode` 并实现 `FixedNodeInterface`。
//! Rust 侧用 trait 表示，`FixedNode: ValueNode` 即 Java 继承链。

use crate::nodes::value_node::ValueNode;

/// 对应 `abstract class FixedNode extends ValueNode implements FixedNodeInterface`。
///
/// 控制流图中位置固定的所有节点的基类。FixedNode 不可浮动——它们必须出现在
/// 控制流的特定位置。
pub trait FixedNode: ValueNode {
    /// 对应 `asFixedNode()`：返回自身作为 FixedNode 引用。
    /// 此方法为 final（不可覆写），确保可去虚化内联。
    fn as_fixed_node(&self) -> &dyn FixedNode
    where
        Self: Sized,
    {
        self
    }
}
