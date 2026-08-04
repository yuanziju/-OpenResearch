// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.graal.compiler.lir.phases`：LIR 阶段。
//!
//! 偏离记录：Java 子包 `lir.phases` 含 LIR 编译阶段（预分配、后分配等）。
//! Rust 侧提供 trait 和基础 struct 定义。

use crate::lir::lir::LIR;

/// 对应 `lir.phases.LIRPhase`：LIR 阶段基类。
///
/// 所有 LIR 编译阶段的基类。
pub trait LIRPhase: std::fmt::Debug {
    /// 对应 `run(LIR)`：运行阶段。
    fn run(&mut self, lir: &mut LIR);

    /// 对应 `getName()`：获取阶段名称。
    fn get_name(&self) -> &str;
}

/// 对应 `lir.phases.LIRPhaseSuite`：LIR 阶段套件。
#[derive(Debug)]
pub struct LIRPhaseSuite {
    /// 阶段列表。
    phases: Vec<Box<dyn LIRPhase>>,
    /// 套件名称。
    name: String,
}

impl LIRPhaseSuite {
    /// 创建阶段套件。
    pub fn new(name: &str) -> Self {
        Self {
            phases: Vec::new(),
            name: name.to_string(),
        }
    }

    /// 添加阶段。
    pub fn add_phase(&mut self, phase: Box<dyn LIRPhase>) {
        self.phases.push(phase);
    }

    /// 运行所有阶段。
    pub fn run(&mut self, lir: &mut LIR) {
        for phase in &mut self.phases {
            phase.run(lir);
        }
    }

    /// 获取阶段数量。
    pub fn phase_count(&self) -> usize {
        self.phases.len()
    }

    /// 获取套件名称。
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

/// 对应 `lir.phases.PreAllocationOptimizationPhase`：预分配优化阶段。
pub trait PreAllocationOptimizationPhase: LIRPhase {}

/// 对应 `lir.phases.PostAllocationOptimizationPhase`：后分配优化阶段。
pub trait PostAllocationOptimizationPhase: LIRPhase {}

/// 对应 `lir.phases.AllocationPhase`：寄存器分配阶段。
pub trait AllocationPhase: LIRPhase {
    /// 获取分配后的寄存器分配映射。
    fn get_allocation_map(&self) -> &dyn AllocationMap;
}

/// 对应 `lir.phases.AllocationMap`：寄存器分配映射。
pub trait AllocationMap {
    /// 获取变量被分配到的寄存器/栈槽。
    fn get_allocation(
        &self,
        var: &crate::lir::variable::Variable,
    ) -> Option<Box<dyn rustci_vm_ci::meta::value::Value>>;
}