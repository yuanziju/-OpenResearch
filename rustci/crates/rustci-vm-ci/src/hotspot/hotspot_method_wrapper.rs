// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2025, Oracle and/or its affiliates. All rights reserved.
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

//! compile0 调度链路的 HotSpot 方法包装器：将 HotSpot 传递的 `*mut c_void` 方法指针包装为
//! `Box<dyn ResolvedJavaMethod>`，供 `CompilationRequest` 构造使用。
//!
//! 偏离记录：
//! - 包装器为 FFI 边界适配层：`*mut c_void` 承载 HotSpot 端的 `Method*` 指针，
//!   实际方法元数据操作（`getCode`/`getConstantPool` 等）由 HotSpot 端 CompilerToVM
//!   提供，Rust 侧仅持有指针并通过 JNI 回传。包装器方法返回默认/空值，对齐编译期合法性。
//! - `DummyJavaType`/`DummySignature`/`DummyConstantPool` 为内部辅助类型，
//!   仅用于满足 trait 边界，不参与实际数据流。

use std::any::Any;
use std::ffi::c_void;
use std::fmt;

use crate::meta::annotated::Annotated;
use crate::meta::annotation_data::AnnotationData;
use crate::meta::constant::Constant;
use crate::meta::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::meta::exception_handler::ExceptionHandler;
use crate::meta::invoke_target::InvokeTarget;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_field::JavaField;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_method::JavaMethod;
use crate::meta::java_reflect::{JavaAnnotation, JavaReflectType};
use crate::meta::java_type::JavaType;
use crate::meta::line_number_table::LineNumberTable;
use crate::meta::local_variable_table::LocalVariableTable;
use crate::meta::meta_util::StackTraceElement;
use crate::meta::modifiers_provider::ModifiersProvider;
use crate::meta::profiling_info::ProfilingInfo;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::resolved_java_type::ResolvedJavaType;
use crate::meta::signature::Signature;
use crate::meta::speculation_log::SpeculationLog;

// =============================================================================
// 内部辅助类型：DummyJavaType / DummySignature / DummyConstantPool / DummySpeculationLog / DummyProfilingInfo
// =============================================================================

/// 内部占位 `JavaType`，包装一个类型名（internal form），用于满足 `HotSpotResolvedJavaMethodWrapper`
/// 的 `get_declaring_class()` / `get_signature()` 返回类型边界。
#[derive(Debug)]
struct DummyJavaType {
    name: String,
}

impl DummyJavaType {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl JavaType for DummyJavaType {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_component_type(&self) -> Option<&dyn JavaType> {
        None
    }

    fn get_array_class(&self) -> Box<dyn JavaType> {
        Box::new(DummyJavaType::new(&format!("[{}", self.name)))
    }

    fn get_java_kind(&self) -> JavaKind {
        if self.name.starts_with('[') {
            JavaKind::Object
        } else {
            match self.name.as_str() {
                "Z" => JavaKind::Boolean,
                "B" => JavaKind::Byte,
                "S" => JavaKind::Short,
                "C" => JavaKind::Char,
                "I" => JavaKind::Int,
                "J" => JavaKind::Long,
                "F" => JavaKind::Float,
                "D" => JavaKind::Double,
                "V" => JavaKind::Void,
                _ => JavaKind::Object,
            }
        }
    }

    fn resolve(&self, _accessing_class: &dyn ResolvedJavaType) -> Box<dyn ResolvedJavaType> {
        panic!("DummyJavaType::resolve not supported")
    }
}

/// 内部占位 `Signature`，用于满足 `HotSpotResolvedJavaMethodWrapper` 的 `get_signature()` 返回类型边界。
#[derive(Debug)]
struct DummySignature {
    param_count: i32,
    return_name: String,
}

impl DummySignature {
    fn new(param_count: i32, return_name: &str) -> Self {
        Self {
            param_count,
            return_name: return_name.to_string(),
        }
    }
}

impl Signature for DummySignature {
    fn get_parameter_count(&self, _receiver: bool) -> i32 {
        self.param_count
    }

    fn get_parameter_type(
        &self,
        _index: i32,
        _accessing_class: Option<&dyn ResolvedJavaType>,
    ) -> Box<dyn JavaType> {
        Box::new(DummyJavaType::new("Ljava/lang/Object;"))
    }

    fn get_return_type(
        &self,
        _accessing_class: Option<&dyn ResolvedJavaType>,
    ) -> Box<dyn JavaType> {
        Box::new(DummyJavaType::new(&self.return_name))
    }
}

/// 内部占位 `ConstantPool`，用于满足 `HotSpotResolvedJavaMethodWrapper` 的 `get_constant_pool()` 返回类型边界。
#[derive(Debug)]
struct DummyConstantPool;

impl ConstantPool for DummyConstantPool {
    fn length(&self) -> i32 {
        0
    }

    fn load_referenced_type(&self, _raw_index: i32, _opcode: i32) {}

    fn lookup_referenced_type(&self, _raw_index: i32, _opcode: i32) -> Box<dyn JavaType> {
        panic!("DummyConstantPool::lookup_referenced_type not supported")
    }

    fn lookup_field(
        &self,
        _raw_index: i32,
        _method: &dyn ResolvedJavaMethod,
        _opcode: i32,
    ) -> Box<dyn JavaField> {
        panic!("DummyConstantPool::lookup_field not supported")
    }

    fn lookup_method_with(
        &self,
        _cpi: i32,
        _opcode: i32,
        _caller: Option<&dyn ResolvedJavaMethod>,
    ) -> Box<dyn JavaMethod> {
        panic!("DummyConstantPool::lookup_method_with not supported")
    }

    fn lookup_bootstrap_method_invocations(
        &self,
        _invoke_dynamic: bool,
    ) -> Vec<Box<dyn crate::meta::constant_pool::BootstrapMethodInvocation>> {
        Vec::new()
    }

    fn lookup_type(&self, _cpi: i32, _opcode: i32) -> Box<dyn JavaType> {
        panic!("DummyConstantPool::lookup_type not supported")
    }

    fn lookup_utf8(&self, _cpi: i32) -> String {
        String::new()
    }

    fn lookup_signature(&self, _cpi: i32) -> Box<dyn Signature> {
        panic!("DummyConstantPool::lookup_signature not supported")
    }

    fn lookup_constant(&self, _cpi: i32) -> Option<ConstantPoolEntry> {
        None
    }

    fn lookup_constant_with(&self, _cpi: i32, _resolve: bool) -> Option<ConstantPoolEntry> {
        None
    }

    fn lookup_appendix(&self, _raw_index: i32, _opcode: i32) -> Option<Box<dyn JavaConstant>> {
        None
    }
}

/// 内部占位 `SpeculationLog`，用于满足 `ResolvedJavaMethod::get_speculation_log()` 返回类型边界。
#[derive(Debug)]
struct DummySpeculationLog;

impl SpeculationLog for DummySpeculationLog {
    fn collect_failed_speculations(&self) {}

    fn may_speculate(&self, _reason: &dyn crate::meta::speculation_log::SpeculationReason) -> bool {
        false
    }

    fn speculate(
        &self,
        _reason: &dyn crate::meta::speculation_log::SpeculationReason,
    ) -> crate::meta::speculation_log::Speculation {
        crate::meta::speculation_log::no_speculation()
    }

    fn has_speculations(&self) -> bool {
        false
    }

    fn lookup_speculation(
        &self,
        _constant: &dyn JavaConstant,
    ) -> crate::meta::speculation_log::Speculation {
        crate::meta::speculation_log::no_speculation()
    }
}

// =============================================================================
// HotSpotResolvedJavaMethodWrapper
// =============================================================================

/// 内部占位 `ProfilingInfo`，返回默认值，用于满足 `ResolvedJavaMethod::get_profiling_info_with()` 返回类型边界。
#[derive(Debug)]
struct DummyProfilingInfo;

impl ProfilingInfo for DummyProfilingInfo {
    fn get_code_size(&self) -> i32 {
        0
    }
    fn get_branch_taken_probability(&self, _bci: i32) -> f64 {
        -1.0
    }
    fn get_switch_probabilities(&self, _bci: i32) -> Option<Vec<f64>> {
        None
    }
    fn get_type_profile(
        &self,
        _bci: i32,
    ) -> Option<&crate::meta::java_type_profile::JavaTypeProfile> {
        None
    }
    fn get_method_profile(
        &self,
        _bci: i32,
    ) -> Option<&crate::meta::java_method_profile::JavaMethodProfile> {
        None
    }
    fn get_exception_seen(&self, _bci: i32) -> crate::meta::tri_state::TriState {
        crate::meta::tri_state::TriState::Unknown
    }
    fn get_null_seen(&self, _bci: i32) -> crate::meta::tri_state::TriState {
        crate::meta::tri_state::TriState::Unknown
    }
    fn get_execution_count(&self, _bci: i32) -> i32 {
        -1
    }
    fn get_deoptimization_count(
        &self,
        _reason: crate::meta::deoptimization::DeoptimizationReason,
    ) -> i32 {
        0
    }
    fn set_compiler_ir_size(&self, _ir_type: &str, _ir_size: i32) -> bool {
        false
    }
    fn get_compiler_ir_size(&self, _ir_type: &str) -> i32 {
        -1
    }
    fn is_mature(&self) -> bool {
        false
    }
    fn set_mature(&self) {}
}

/// 包装 HotSpot 传递的 `Method*` 指针，实现 `ResolvedJavaMethod` trait。
///
/// 该包装器为 FFI 边界适配层：`method_ptr` 承载 HotSpot 端的原始方法指针，
/// 实际方法元数据操作由 HotSpot 端 CompilerToVM 通过 JNI 提供。
/// 包装器方法返回默认/空值以保证编译期合法性。
pub struct HotSpotResolvedJavaMethodWrapper {
    /// HotSpot 端 `Method*` 指针（JNI 层的 `jlong` 句柄）。
    pub method_ptr: *mut c_void,
    /// 方法名（从 HotSpot 端获取后缓存）。
    name: String,
    /// 声明类（占位 `JavaType`）。
    declaring_class: DummyJavaType,
    /// 签名（占位 `Signature`）。
    signature: DummySignature,
    /// 常量池（占位 `ConstantPool`）。
    constant_pool: DummyConstantPool,
}

impl HotSpotResolvedJavaMethodWrapper {
    /// 构造包装器。
    pub fn new(method_ptr: *mut c_void, name: &str) -> Self {
        Self {
            method_ptr,
            name: name.to_string(),
            declaring_class: DummyJavaType::new("Ljava/lang/Object;"),
            signature: DummySignature::new(0, "V"),
            constant_pool: DummyConstantPool,
        }
    }
}

impl fmt::Debug for HotSpotResolvedJavaMethodWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HotSpotResolvedJavaMethodWrapper")
            .field("method_ptr", &self.method_ptr)
            .field("name", &self.name)
            .finish()
    }
}

impl InvokeTarget for HotSpotResolvedJavaMethodWrapper {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ModifiersProvider for HotSpotResolvedJavaMethodWrapper {
    fn get_modifiers(&self) -> i32 {
        0
    }
}

impl Annotated for HotSpotResolvedJavaMethodWrapper {
    fn get_annotation_data(&self, _type_: &dyn ResolvedJavaType) -> Option<AnnotationData> {
        None
    }

    fn get_annotation_data_many(
        &self,
        _type1: &dyn ResolvedJavaType,
        _type2: &dyn ResolvedJavaType,
        _types: &[&dyn ResolvedJavaType],
    ) -> Vec<AnnotationData> {
        Vec::new()
    }
}

impl JavaMethod for HotSpotResolvedJavaMethodWrapper {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_declaring_class(&self) -> &dyn JavaType {
        &self.declaring_class
    }

    fn get_signature(&self) -> &dyn Signature {
        &self.signature
    }

    fn as_resolved_java_method(&self) -> Option<&dyn ResolvedJavaMethod> {
        Some(self)
    }
}

impl ResolvedJavaMethod for HotSpotResolvedJavaMethodWrapper {
    fn get_code(&self) -> Option<Vec<u8>> {
        None
    }

    fn get_code_size(&self) -> i32 {
        0
    }

    fn get_max_locals(&self) -> i32 {
        0
    }

    fn get_max_stack_size(&self) -> i32 {
        0
    }

    fn is_synthetic(&self) -> bool {
        false
    }

    fn is_var_args(&self) -> bool {
        false
    }

    fn is_bridge(&self) -> bool {
        false
    }

    fn is_default(&self) -> bool {
        false
    }

    fn is_declared(&self) -> bool {
        true
    }

    fn is_class_initializer(&self) -> bool {
        false
    }

    fn is_constructor(&self) -> bool {
        false
    }

    fn can_be_statically_bound(&self) -> bool {
        false
    }

    fn get_exception_handlers(&self) -> Vec<ExceptionHandler> {
        Vec::new()
    }

    fn as_stack_trace_element(&self, _bci: i32) -> StackTraceElement {
        StackTraceElement::new(&self.name, "Unknown", Some("Unknown".to_string()), -1)
    }

    fn get_profiling_info_with(
        &self,
        _include_normal: bool,
        _include_osr: bool,
    ) -> Box<dyn ProfilingInfo> {
        Box::new(DummyProfilingInfo)
    }

    fn reprofile(&self) {}

    fn get_constant_pool(&self) -> &dyn ConstantPool {
        &self.constant_pool
    }

    fn get_parameter_annotations(&self) -> Vec<Vec<Box<dyn JavaAnnotation>>> {
        Vec::new()
    }

    fn get_generic_parameter_types(&self) -> Vec<Box<dyn JavaReflectType>> {
        Vec::new()
    }

    fn can_be_inlined(&self) -> bool {
        false
    }

    fn has_never_inline_directive(&self) -> bool {
        false
    }

    fn should_be_inlined(&self) -> bool {
        false
    }

    fn get_line_number_table(&self) -> Option<LineNumberTable> {
        None
    }

    fn get_local_variable_table(&self) -> Option<LocalVariableTable> {
        None
    }

    fn get_encoding(&self) -> Box<dyn Constant> {
        Box::new(crate::meta::java_constant::for_illegal())
    }

    fn is_in_virtual_method_table(&self, _resolved: &dyn ResolvedJavaType) -> bool {
        false
    }

    fn get_speculation_log(&self) -> Box<dyn SpeculationLog> {
        Box::new(DummySpeculationLog)
    }
}
