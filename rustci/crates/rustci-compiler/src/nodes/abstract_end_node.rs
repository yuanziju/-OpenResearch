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

//! 镜像 `jdk.graal.compiler.nodes.AbstractEndNode`：控制流合并中结束节点的抽象基类。
//!
//! 偏离记录：Java `AbstractEndNode` 继承 `FixedNode` 并实现 `LIRLowerable`。
//! Rust 侧为 trait。

use crate::nodes::fixed_node::FixedNode;

/// 对应 `abstract class AbstractEndNode extends FixedNode implements LIRLowerable`。
///
/// 控制流合并（merge）中结束节点的抽象基类。EndNode 和 LoopEndNode 都继承自此。
pub trait AbstractEndNode: FixedNode {
    /// 对应 `merge()`：返回此结束节点所连接的合并节点。
    fn merge(&self) -> Option<&dyn crate::nodes::abstract_merge_node::AbstractMergeNode>;
}
