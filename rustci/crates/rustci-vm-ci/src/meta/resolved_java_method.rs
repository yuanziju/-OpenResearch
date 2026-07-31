// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ResolvedJavaMethod`：已解析方法接口。
//!
//! 偏离记录：
//! - Java `ResolvedJavaMethod extends JavaMethod, InvokeTarget, ModifiersProvider, AnnotatedElement, Annotated`
//!   → Rust `trait ResolvedJavaMethod: JavaMethod + InvokeTarget + ModifiersProvider + Annotated`
//!   （跳过 `AnnotatedElement`，同 `ResolvedJavaField`）。
//! - Java `getDeclaringClass()` 协变覆写（`JavaType` → `ResolvedJavaType`）：Rust 不支持协变返回，
//!   继承 `JavaMethod::get_declaring_class() -> &dyn JavaType`。
//! - Java `getProfilingInfo()` / `getProfilingInfo(boolean, boolean)` 重载 → Rust
//!   `get_profiling_info()`（默认，调 `get_profiling_info_with(true, true)`）/
//!   `get_profiling_info_with(bool, bool)`。
//! - Java 嵌套 `class Parameter` → Rust 同文件 `pub struct Parameter`。
//! - `getParameterAnnotation`/`getParameterAnnotations(Class<T>)` 默认方法依赖 Java 反射泛型 `<T extends Annotation>`，
//!   Rust 侧无等价物，跳过（非抽象方法，不影响 trait 可实现性）。
//! - `UnsupportedOperationException`（Java 非受检）→ Rust `panic!`。

use crate::meta::annotated::Annotated;
use crate::meta::constant::Constant;
use crate::meta::constant_pool::ConstantPool;
use crate::meta::exception_handler::ExceptionHandler;
use crate::meta::invoke_target::InvokeTarget;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_method::JavaMethod;
use crate::meta::java_reflect::{JavaAnnotation, JavaReflectType};
use crate::meta::java_type::JavaType;
use crate::meta::line_number_table::LineNumberTable;
use crate::meta::local_variable_table::LocalVariableTable;
use crate::meta::meta_util::StackTraceElement;
use crate::meta::modifiers_provider::ModifiersProvider;
use crate::meta::profiling_info::ProfilingInfo;
use crate::meta::speculation_log::SpeculationLog;

/// 对应 `interface ResolvedJavaMethod extends JavaMethod, InvokeTarget, ModifiersProvider, Annotated`。
pub trait ResolvedJavaMethod: JavaMethod + InvokeTarget + ModifiersProvider + Annotated {
    /// 对应 `getCode()`：返回 `None` 当 `get_code_size() <= 0`。
    fn get_code(&self) -> Option<Vec<u8>>;

    /// 对应 `getCodeSize()`。
    fn get_code_size(&self) -> i32;

    /// 对应 `getMaxLocals()`。
    fn get_max_locals(&self) -> i32;

    /// 对应 `getMaxStackSize()`。
    fn get_max_stack_size(&self) -> i32;

    /// 对应 `isFinal()`（默认方法）。
    fn is_final(&self) -> bool {
        self.is_final_flag_set()
    }

    /// 对应 `isSynthetic()`。
    fn is_synthetic(&self) -> bool;

    /// 对应 `isVarArgs()`。
    fn is_var_args(&self) -> bool;

    /// 对应 `isBridge()`。
    fn is_bridge(&self) -> bool;

    /// 对应 `isDefault()`。
    fn is_default(&self) -> bool;

    /// 对应 `isDeclared()`。
    fn is_declared(&self) -> bool;

    /// 对应 `isClassInitializer()`。
    fn is_class_initializer(&self) -> bool;

    /// 对应 `isConstructor()`。
    fn is_constructor(&self) -> bool;

    /// 对应 `canBeStaticallyBound()`。
    fn can_be_statically_bound(&self) -> bool;

    /// 对应 `getExceptionHandlers()`。
    fn get_exception_handlers(&self) -> Vec<ExceptionHandler>;

    /// 对应 `asStackTraceElement(int bci)`。
    fn as_stack_trace_element(&self, bci: i32) -> StackTraceElement;

    /// 对应 `getProfilingInfo()`（默认方法）。
    fn get_profiling_info(&self) -> Box<dyn ProfilingInfo> {
        self.get_profiling_info_with(true, true)
    }

    /// 对应 `getProfilingInfo(boolean includeNormal, boolean includeOSR)`。
    fn get_profiling_info_with(
        &self,
        include_normal: bool,
        include_osr: bool,
    ) -> Box<dyn ProfilingInfo>;

    /// 对应 `reprofile()`。
    fn reprofile(&self);

    /// 对应 `getConstantPool()`。
    fn get_constant_pool(&self) -> &dyn ConstantPool;

    /// 对应 `getParameters()`（默认返回 `None`）。
    fn get_parameters(&self) -> Option<Vec<Parameter>> {
        None
    }

    /// 对应 `getParameterAnnotations()`：每参数的注解列表。
    fn get_parameter_annotations(&self) -> Vec<Vec<Box<dyn JavaAnnotation>>>;

    /// 对应 `getGenericParameterTypes()`。
    fn get_generic_parameter_types(&self) -> Vec<Box<dyn JavaReflectType>>;

    /// 对应 `canBeInlined()`。
    fn can_be_inlined(&self) -> bool;

    /// 对应 `hasNeverInlineDirective()`。
    fn has_never_inline_directive(&self) -> bool;

    /// 对应 `shouldBeInlined()`。
    fn should_be_inlined(&self) -> bool;

    /// 对应 `getLineNumberTable()`：无表返回 `None`。
    fn get_line_number_table(&self) -> Option<LineNumberTable>;

    /// 对应 `getLocalVariableTable()`：无表返回 `None`。
    fn get_local_variable_table(&self) -> Option<LocalVariableTable>;

    /// 对应 `getEncoding()`。
    fn get_encoding(&self) -> Box<dyn Constant>;

    /// 对应 `isInVirtualMethodTable(ResolvedJavaType)`。
    fn is_in_virtual_method_table(
        &self,
        resolved: &dyn crate::meta::resolved_java_type::ResolvedJavaType,
    ) -> bool;

    /// 对应 `hasBytecodes()`（默认方法）。
    fn has_bytecodes(&self) -> bool {
        self.get_code_size() > 0
    }

    /// 对应 `hasReceiver()`（默认方法）。
    fn has_receiver(&self) -> bool {
        !self.is_static()
    }

    /// 对应 `isJavaLangObjectInit()`（默认方法）。
    fn is_java_lang_object_init(&self) -> bool {
        // getDeclaringClass() 返回 &dyn JavaType，无法直接调 isJavaLangObject；
        // 实现可覆写。默认 false（对齐 Java：需 declaringClass.isJavaLangObject() && name=="<init>"）。
        false
    }

    /// 对应 `isScoped()`（默认抛 `UnsupportedOperationException`）。
    fn is_scoped(&self) -> bool {
        panic!("UnsupportedOperationException");
    }

    /// 对应 `getSpeculationLog()`。
    fn get_speculation_log(&self) -> Box<dyn SpeculationLog>;

    /// Rust 增设：覆写 `JavaMethod::as_resolved_java_method` 返回 `Some(self)`。
    fn as_resolved_java_method(&self) -> Option<&dyn ResolvedJavaMethod>
    where
        Self: Sized,
    {
        Some(self)
    }
}

/// 对应 `ResolvedJavaMethod.Parameter`：方法参数描述。
pub struct Parameter {
    name: Option<String>,
    method: Box<dyn ResolvedJavaMethod>,
    modifiers: i32,
    index: i32,
}

impl Parameter {
    /// 对应 `Parameter(String name, int modifiers, ResolvedJavaMethod method, int index)`。
    pub fn new(
        name: Option<String>,
        modifiers: i32,
        method: Box<dyn ResolvedJavaMethod>,
        index: i32,
    ) -> Self {
        debug_assert!(name.as_ref().map_or(true, |n| !n.is_empty()));
        Self {
            name,
            method,
            modifiers,
            index,
        }
    }

    /// 对应 `getName()`。
    pub fn get_name(&self) -> String {
        match &self.name {
            None => format!("arg{}", self.index),
            Some(n) => n.clone(),
        }
    }

    /// 对应 `getDeclaringMethod()`。
    pub fn get_declaring_method(&self) -> &dyn ResolvedJavaMethod {
        self.method.as_ref()
    }

    /// 对应 `getModifiers()`。
    pub fn get_modifiers(&self) -> i32 {
        self.modifiers
    }

    /// 对应 `getKind()`。
    pub fn get_kind(&self) -> JavaKind {
        self.method.get_signature().get_parameter_kind(self.index)
    }

    /// 对应 `getType()`。
    pub fn get_type(&self) -> Box<dyn JavaType> {
        self.method
            .get_signature()
            .get_parameter_type(self.index, None)
    }

    /// 对应 `isNamePresent()`。
    pub fn is_name_present(&self) -> bool {
        self.name.is_some()
    }

    /// 对应 `isVarArgs()`。
    pub fn is_var_args(&self) -> bool {
        self.method.is_var_args()
            && self.index == self.method.get_signature().get_parameter_count(false) - 1
    }
}

impl PartialEq for Parameter {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&*self.method, &*other.method) && self.index == other.index
    }
}

impl Eq for Parameter {}

impl std::hash::Hash for Parameter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::addr_of!(*self.method).hash(state);
        self.index.hash(state);
    }
}
