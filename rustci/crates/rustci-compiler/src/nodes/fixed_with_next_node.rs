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

//! 镜像 `jdk.graal.compiler.nodes.FixedWithNextNode`：控制流图中具有直接后继的固定节点。
//!
//! 偏离记录：Java `FixedWithNextNode` 继承 `FixedNode` 并实现 `FixedWithNextNodeInterface`。
//! Rust 侧用 trait 表示，`FixedWithNextNode: FixedNode`。

use crate::nodes::fixed_node::FixedNode;

/// 对应 `abstract class FixedWithNextNode extends FixedNode implements FixedWithNextNodeInterface`。
///
/// 控制流图中位置固定且具有直接后继的所有节点的基类。
pub trait FixedWithNextNode: FixedNode {
    /// 对应 `next()`：获取直接后继。
    fn next(&self) -> Option<&dyn FixedNode>;

    /// 对应 `setNext(FixedNode)`：设置直接后继。
    fn set_next(&mut self, next: Option<Box<dyn FixedNode>>);

    /// 对应 `asFixedWithNextNode()`：返回自身作为 FixedWithNextNode 引用。
    fn as_fixed_with_next_node(&self) -> &dyn FixedWithNextNode
    where
        Self: Sized,
    {
        self
    }
}
