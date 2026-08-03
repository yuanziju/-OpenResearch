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

//! 镜像 `jdk.graal.compiler.nodes.OptimizationLogImpl`：优化日志的"慢"实现。
//!
//! 偏离记录：Java `OptimizationLogImpl` 实现 `OptimizationLog` 接口，执行实际日志记录。
//! Rust 侧为具体 struct。

use std::collections::HashMap;

use crate::nodes::optimization_log::{OptimizationEntry, OptimizationLog};

/// 对应 `class OptimizationLogImpl implements OptimizationLog`。
///
/// 统一计数、日志和转储的"慢"实现。执行实际的日志记录操作。
/// 如果所有日志特性均未启用，应使用虚拟实现以减少运行时开销。
#[derive(Debug, Clone)]
pub struct OptimizationLogImpl {
    /// 优化条目列表。
    pub entries: Vec<OptimizationLogEntry>,
    /// 是否启用日志。
    pub enabled: bool,
}

/// 对应单个优化日志条目。
#[derive(Debug, Clone)]
pub struct OptimizationLogEntry {
    /// 优化类名。
    pub optimization_class: String,
    /// 事件名称。
    pub event_name: String,
    /// 属性。
    pub properties: HashMap<String, String>,
}

impl OptimizationLogEntry {
    pub fn new(optimization_class: &str, event_name: &str) -> Self {
        OptimizationLogEntry {
            optimization_class: optimization_class.to_string(),
            event_name: event_name.to_string(),
            properties: HashMap::new(),
        }
    }
}

impl OptimizationEntry for OptimizationLogEntry {
    fn with_property(&mut self, key: &str, value: String) {
        self.properties.insert(key.to_string(), value);
    }

    fn report(&mut self, _optimization_class: &str, _event_name: &str) {}
}

impl OptimizationLogImpl {
    /// 常量属性键。
    pub const METHOD_NAME_PROPERTY: &'static str = "methodName";
    pub const OPTIMIZATION_NAME_PROPERTY: &'static str = "optimizationName";
    pub const EVENT_NAME_PROPERTY: &'static str = "eventName";
    pub const POSITION_PROPERTY: &'static str = "position";
    pub const PHASE_NAME_PROPERTY: &'static str = "phaseName";
    pub const OPTIMIZATIONS_PROPERTY: &'static str = "optimizations";
    pub const CALLSITE_BCI_PROPERTY: &'static str = "callsiteBci";

    /// 创建一个新的 OptimizationLogImpl。
    pub fn new(enabled: bool) -> Self {
        OptimizationLogImpl {
            entries: Vec::new(),
            enabled,
        }
    }

    /// 添加一个优化条目。
    pub fn add_entry(
        &mut self,
        optimization_class: &str,
        event_name: &str,
    ) -> &mut OptimizationLogEntry {
        let entry = OptimizationLogEntry::new(optimization_class, event_name);
        self.entries.push(entry);
        self.entries.last_mut().unwrap()
    }

    /// 获取所有条目。
    pub fn get_entries(&self) -> &[OptimizationLogEntry] {
        &self.entries
    }
}

impl OptimizationLog for OptimizationLogImpl {
    fn create_entry(&self) -> Box<dyn OptimizationEntry> {
        Box::new(OptimizationLogEntry::new("", ""))
    }
}
