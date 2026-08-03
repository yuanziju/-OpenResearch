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

//! 镜像 `jdk.graal.compiler.nodes.OptimizationLog`：优化日志接口。
//!
//! 偏离记录：Java `OptimizationLog` 是接口，包含 `OptimizationTreeNode`、
//! `OptimizationEntry`、`OptimizationEntryDummy` 等内部类型。Rust 侧用 trait 表示。

use std::collections::HashMap;

/// 对应 `interface OptimizationLog`。
///
/// 统一优化阶段的计数、日志和转储。若启用，收集单次编译中执行的优化信息，
/// 并将其转储到标准输出、JSON 文件和/或 IGV。
pub trait OptimizationLog {
    /// 对应 `OptimizationEntry` 接口：描述一次已执行的优化。
    fn create_entry(&self) -> Box<dyn OptimizationEntry>;
}

/// 对应 `OptimizationLog.OptimizationEntry` 接口。
pub trait OptimizationEntry {
    /// 对应 `withProperty(String, Object)`：添加属性。
    fn with_property(&mut self, key: &str, value: String);

    /// 对应 `report(Class, String, Node)`：在 DETAILED_LEVEL 报告优化。
    fn report(&mut self, optimization_class: &str, event_name: &str);
}

/// 对应 `OptimizationLog.OptimizationEntryDummy`：不执行操作的虚拟条目。
#[derive(Debug, Clone)]
pub struct OptimizationEntryDummy {
    properties: HashMap<String, String>,
}

impl OptimizationEntryDummy {
    pub fn new() -> Self {
        OptimizationEntryDummy {
            properties: HashMap::new(),
        }
    }
}

impl Default for OptimizationEntryDummy {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationEntry for OptimizationEntryDummy {
    fn with_property(&mut self, key: &str, value: String) {
        self.properties.insert(key.to_string(), value);
    }

    fn report(&mut self, _optimization_class: &str, _event_name: &str) {}
}
