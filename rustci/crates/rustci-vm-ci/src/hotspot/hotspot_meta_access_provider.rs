// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2019, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotMetaAccessProvider`：HotSpot 元数据访问提供者。
//!
//! 偏离记录：
//! - Java `class HotSpotMetaAccessProvider implements MetaAccessProvider` →
//!   Rust `pub trait HotSpotMetaAccessProvider: MetaAccessProvider`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use crate::meta::deoptimization::{DeoptimizationAction, DeoptimizationReason};
use crate::meta::meta_access::MetaAccessProvider;

/// 对应 `class HotSpotMetaAccessProvider implements MetaAccessProvider`。
pub trait HotSpotMetaAccessProvider: MetaAccessProvider {
    /// 对应 `int convertDeoptAction(DeoptimizationAction)`。
    fn convert_deopt_action(&self, action: DeoptimizationAction) -> i32;

    /// 对应 `DeoptimizationAction convertDeoptAction(int)`。
    fn convert_deopt_action_from_int(&self, action: i32) -> DeoptimizationAction;

    /// 对应 `int convertDeoptReason(DeoptimizationReason)`。
    fn convert_deopt_reason(&self, reason: DeoptimizationReason) -> i32;

    /// 对应 `DeoptimizationReason convertDeoptReason(int)`。
    fn convert_deopt_reason_from_int(&self, reason: i32) -> DeoptimizationReason;

    /// 对应 `int computeArrayAllocationSize(int, int, int)`。
    fn compute_array_allocation_size(
        &self,
        length: i32,
        header_size: i32,
        log2_element_size: i32,
    ) -> i32;
}
