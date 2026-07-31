// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES IN THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit Oracle if you need additional information or have any
 * questions.
 */

//! T4 最小 mock 测试：证明核心 trait 可被实现并通过编译。
//!
//! 覆盖：
//! - `Signature` trait 的 mock 实现（`MockSignature`）。
//! - `UnresolvedJavaType`（既有 `JavaType` 实现）的基本行为。
//! - `UnresolvedJavaMethod`（既有 `JavaMethod` 实现）配合 mock `Signature` 的构造与格式化。
//! - `DefaultProfilingInfo`（既有 `ProfilingInfo` 实现）的缓存与默认值行为。

use crate::meta::default_profiling_info::DefaultProfilingInfo;
use crate::meta::deoptimization::DeoptimizationReason;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_method::JavaMethod;
use crate::meta::java_type::JavaType;
use crate::meta::profiling_info::ProfilingInfo;
use crate::meta::signature::Signature;
use crate::meta::tri_state::TriState;
use crate::meta::unresolved_java_method::UnresolvedJavaMethod;
use crate::meta::unresolved_java_type::UnresolvedJavaType;

/// 最小 mock `Signature`：固定参数类型（`I`）与返回类型（`V`）。
struct MockSignature {
    param_count: i32,
}

impl Signature for MockSignature {
    fn get_parameter_count(&self, receiver: bool) -> i32 {
        self.param_count + if receiver { 1 } else { 0 }
    }

    fn get_parameter_type(
        &self,
        _index: i32,
        _accessing_class: Option<&dyn crate::meta::resolved_java_type::ResolvedJavaType>,
    ) -> Box<dyn JavaType> {
        Box::new(UnresolvedJavaType::create("I"))
    }

    fn get_return_type(
        &self,
        _accessing_class: Option<&dyn crate::meta::resolved_java_type::ResolvedJavaType>,
    ) -> Box<dyn JavaType> {
        Box::new(UnresolvedJavaType::create("V"))
    }
}

#[test]
fn unresolved_java_type_basic() {
    // 验证 `JavaType` trait 实现基本行为。
    let t = UnresolvedJavaType::create("Ljava/lang/Object;");
    assert_eq!(t.get_name(), "Ljava/lang/Object;");
    assert_eq!(t.get_java_kind(), JavaKind::Object);
    assert!(t.get_component_type().is_none());
    assert!(!t.is_array());

    // 数组类型：组件类型预计算。
    let arr = UnresolvedJavaType::create("[Ljava/lang/Object;");
    assert_eq!(arr.get_name(), "[Ljava/lang/Object;");
    assert!(arr.is_array());
    let comp = arr.get_component_type().expect("array has component type");
    assert_eq!(comp.get_name(), "Ljava/lang/Object;");

    // elemental type：数组降维到元素类型。
    assert_eq!(arr.get_elemental_type().get_name(), "Ljava/lang/Object;");

    // 数组类的数组类：前缀 `[`。
    let arr_of_arr = arr.get_array_class();
    assert_eq!(arr_of_arr.get_name(), "[[Ljava/lang/Object;");

    // to_java_name / to_java_name_qualified。
    assert_eq!(t.to_java_name(), "java.lang.Object");
    assert_eq!(t.to_class_name(), "java.lang.Object");

    // PartialEq：按 name 相等。
    let t2 = UnresolvedJavaType::create("Ljava/lang/Object;");
    assert_eq!(&t, &t2);
    let t3 = UnresolvedJavaType::create("I");
    assert_ne!(&t, &t3);
}

#[test]
fn unresolved_java_method_with_mock_signature() {
    // 验证 `JavaMethod` trait 实现配合 mock `Signature` 可构造与格式化。
    let holder = Box::new(UnresolvedJavaType::create("Ljava/lang/String;"));
    let sig = Box::new(MockSignature { param_count: 1 });
    let m = UnresolvedJavaMethod::without_cause("indexOf", sig, holder);

    assert_eq!(m.get_name(), "indexOf");
    assert_eq!(m.get_declaring_class().get_name(), "Ljava/lang/String;");
    assert_eq!(m.get_signature().get_parameter_count(false), 1);
    assert_eq!(m.get_signature().get_parameter_count(true), 2);
    // `UnresolvedJavaType.getJavaKind()` 恒返回 `Object`（对齐 Java 行为），故 return kind 为 Object。
    assert_eq!(m.get_signature().get_return_kind(), JavaKind::Object);
    assert_eq!(m.get_signature().to_method_descriptor(), "(I)V");

    // format：`%H.%n(%p)%r` → holder.name(params)return。
    let formatted = m.format("%H.%n(%p)%r");
    assert_eq!(formatted, "java.lang.String.indexOf(int)void");

    // cause 为 None。
    assert!(m.get_cause().is_none());
}

#[test]
fn default_profiling_info_behavior() {
    // 验证 `ProfilingInfo` trait 实现的默认值与缓存行为。
    let info = DefaultProfilingInfo::get(TriState::True);
    assert_eq!(info.get_code_size(), 0);
    assert_eq!(info.get_exception_seen(0), TriState::True);
    assert_eq!(info.get_null_seen(0), TriState::Unknown);
    assert_eq!(info.get_branch_taken_probability(0), -1.0);
    assert_eq!(info.get_execution_count(0), -1);
    assert_eq!(info.get_switch_probabilities(0), None);
    assert!(info.get_type_profile(0).is_none());
    assert!(info.get_method_profile(0).is_none());
    assert!(!info.is_mature());

    // 各 deopt reason 计数均为 0。
    for reason in DeoptimizationReason::all() {
        assert_eq!(info.get_deoptimization_count(*reason), 0);
    }

    // 缓存：同一 TriState 返回同一实例。
    let info2 = DefaultProfilingInfo::get(TriState::True);
    assert!(std::ptr::eq(info, info2));

    // 不同 TriState 返回不同实例。
    let info_false = DefaultProfilingInfo::get(TriState::False);
    assert!(!std::ptr::eq(info, info_false));
    assert_eq!(info_false.get_exception_seen(0), TriState::False);

    // toString：code_size=0 → to_string_with 返回空 → "DefaultProfilingInfo<>"。
    assert_eq!(info.to_string(), "DefaultProfilingInfo<>");
}

#[test]
fn tri_state_display_and_name() {
    // 验证 TriState 的 Display（对齐 Java Enum.name）。
    assert_eq!(TriState::True.to_string(), "TRUE");
    assert_eq!(TriState::False.to_string(), "FALSE");
    assert_eq!(TriState::Unknown.to_string(), "UNKNOWN");
    assert_eq!(TriState::True.name(), "TRUE");
}
