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

//! 镜像 `jdk.vm.ci.meta.ResolvedJavaType`：已解析类型接口。
//!
//! 偏离记录：
//! - Java `ResolvedJavaType extends JavaType, ModifiersProvider, AnnotatedElement, Annotated`
//!   → Rust `trait ResolvedJavaType: JavaType + ModifiersProvider + Annotated`（跳过
//!   `AnnotatedElement`，其方法依赖 Java 反射 `Annotation`/`Class` 类型，Rust 侧为不透明
//!   标记，非本期 T4 范围；同 `ResolvedJavaMethod`/`ResolvedJavaField`）。
//! - Java 协变覆写：`getComponentType()`/`getElementalType()`/`getArrayClass()`/`getSuperclass()`
//!   等返回 `ResolvedJavaType`（JavaType 返回 `JavaType`）。Rust 不支持 trait 方法协变返回，
//!   故新增 `resolved_*` 命名方法返回 `&dyn ResolvedJavaType`/`Box<dyn ResolvedJavaType>`，
//!   与超 trait 的 `&dyn JavaType`/`Box<dyn JavaType>` 方法并存。`getElementalType` 在超
//!   trait 是默认方法返回 `&dyn JavaType`；此处新增 `get_elemental_type_resolved` 返回
//!   `&dyn ResolvedJavaType`，对应 Java 协变覆写后的 `getElementalType()`。
//! - Java `getHostClass()`/`getSingleImplementor()`/`getSuperclass()` 等 nullable 返回 →
//!   `Option<Box<dyn ResolvedJavaType>>`/`Option<&dyn ResolvedJavaType>`（对齐 Java
//!   nullable 引用语义）。
//! - Java `findLeastCommonAncestor` 返回 `ResolvedJavaType`（可空）→ `Option<Box<dyn ResolvedJavaType>>`。
//! - Java 数组返回 `ResolvedJavaType[]`/`ResolvedJavaMethod[]`/`ResolvedJavaField[]` →
//!   `Vec<Box<dyn ResolvedJavaType>>` 等。
//! - Java `findInstanceFieldWithOffset` 返回 nullable → `Option<Box<dyn ResolvedJavaField>>`。
//! - Java `resolveMethod` 返回 nullable → `Option<Box<dyn ResolvedJavaMethod>>`。
//! - `link()`/`hasDefaultMethods()`/`declaresDefaultMethods()`/`lookupType`/`resolveField`
//!   默认方法抛 `UnsupportedOperationException` 或返回 `null` → Rust 默认 `panic!` 或 `None`，
//!   对齐 Java 行为。
//! - `findLeastCommonAncestor` 文档注明"primitive 时返回 null"——返回 `Option` 即可表达。
//! - Java `findLeafConcreteSubtype`/`findUniqueConcreteMethod` 返回
//!   `AssumptionResult<ResolvedJavaType>`/`AssumptionResult<ResolvedJavaMethod>` →
//!   `AssumptionResult<Box<dyn ResolvedJavaType>>`/`AssumptionResult<Box<dyn ResolvedJavaMethod>>`。
//! - `UnsupportedOperationException`（Java 非受检）→ Rust `panic!`。

use crate::meta::annotated::Annotated;
use crate::meta::assumptions::AssumptionResult;
use crate::meta::java_constant::JavaConstant;
use crate::meta::java_kind::JavaKind;
use crate::meta::java_type::JavaType;
use crate::meta::modifiers_provider::ModifiersProvider;
use crate::meta::resolved_java_field::ResolvedJavaField;
use crate::meta::resolved_java_method::ResolvedJavaMethod;
use crate::meta::signature::Signature;
use crate::meta::unresolved_java_field::UnresolvedJavaField;
use crate::meta::unresolved_java_type::UnresolvedJavaType;

/// 对应 `interface ResolvedJavaType extends JavaType, ModifiersProvider, Annotated`。
pub trait ResolvedJavaType: JavaType + ModifiersProvider + Annotated {
    /// 对应 `hasFinalizer()`。
    fn has_finalizer(&self) -> bool;

    /// 对应 `hasFinalizableSubclass()`：返回 `AssumptionResult<Boolean>`。
    fn has_finalizable_subclass(&self) -> AssumptionResult<bool>;

    /// 对应 `isInterface()`（覆写 `ModifiersProvider.isInterface`）。
    fn is_interface(&self) -> bool;

    /// 对应 `isInstanceClass()`。
    fn is_instance_class(&self) -> bool;

    /// 对应 `isPrimitive()`。
    fn is_primitive(&self) -> bool;

    /// 对应 `isLeaf()`（默认方法）。
    ///
    /// 偏离记录：增设 `Self: Sized` 约束——方法体调 `get_elemental_type_resolved`（需 `Self: Sized`）。
    fn is_leaf(&self) -> bool
    where
        Self: Sized,
    {
        self.get_elemental_type_resolved().is_final_flag_set()
    }

    /// 对应 `isEnum()`。
    fn is_enum(&self) -> bool;

    /// 对应 `isInitialized()`。
    fn is_initialized(&self) -> bool;

    /// 对应 `initialize()`。
    fn initialize(&self);

    /// 对应 `isLinked()`。
    fn is_linked(&self) -> bool;

    /// 对应 `link()`（默认抛 `UnsupportedOperationException`）。
    fn link(&self) {
        panic!("UnsupportedOperationException: link is unsupported");
    }

    /// 对应 `hasDefaultMethods()`（默认抛 `UnsupportedOperationException`）。
    fn has_default_methods(&self) -> bool {
        panic!("UnsupportedOperationException: hasDefaultMethods is unsupported");
    }

    /// 对应 `declaresDefaultMethods()`（默认抛 `UnsupportedOperationException`）。
    fn declares_default_methods(&self) -> bool {
        panic!("UnsupportedOperationException: declaresDefaultMethods is unsupported");
    }

    /// 对应 `isAssignableFrom(ResolvedJavaType)`。
    fn is_assignable_from(&self, other: &dyn ResolvedJavaType) -> bool;

    /// 对应 `getHostClass()`（默认返回 `None`，对应 Java 返回 `null`）。
    fn get_host_class(&self) -> Option<Box<dyn ResolvedJavaType>> {
        None
    }

    /// 对应 `isJavaLangObject()`（默认方法）。
    fn is_java_lang_object(&self) -> bool {
        // `is_interface` 在 `ModifiersProvider`（默认）与 `ResolvedJavaType`（覆写）中均有，
        // 显式消歧取 `ResolvedJavaType` 覆写版（对齐 Java 协变覆写语义）。
        self.get_superclass().is_none()
            && !ResolvedJavaType::is_interface(self)
            && self.get_java_kind() == JavaKind::Object
    }

    /// 对应 `isInstance(JavaConstant)`。
    fn is_instance(&self, obj: &dyn JavaConstant) -> bool;

    /// 对应 `getSuperclass()`：返回 `Option` 对齐 Java nullable 语义。
    fn get_superclass(&self) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `getInterfaces()`。
    fn get_interfaces(&self) -> Vec<Box<dyn ResolvedJavaType>>;

    /// 对应 `getSingleImplementor()`：返回 `Option` 对齐 Java nullable 语义。
    fn get_single_implementor(&self) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `findLeastCommonAncestor(ResolvedJavaType)`：返回 `Option` 对齐 Java nullable 语义。
    fn find_least_common_ancestor(
        &self,
        other_type: &dyn ResolvedJavaType,
    ) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `findLeafConcreteSubtype()`。
    fn find_leaf_concrete_subtype(&self) -> AssumptionResult<Option<Box<dyn ResolvedJavaType>>>;

    /// Rust 增设：对应 Java 协变覆写 `getComponentType()`（返回 `ResolvedJavaType`）。
    /// 与 `JavaType::get_component_type` 并存。
    fn get_component_type_resolved(&self) -> Option<&dyn ResolvedJavaType>;

    /// Rust 增设：对应 Java 协变覆写 `getElementalType()`（返回 `ResolvedJavaType`）。
    fn get_elemental_type_resolved(&self) -> &dyn ResolvedJavaType
    where
        Self: Sized,
    {
        let mut t: &dyn ResolvedJavaType = self;
        while let Some(c) = t.get_component_type_resolved() {
            t = c;
        }
        t
    }

    /// Rust 增设：对应 Java 协变覆写 `getArrayClass()`（返回 `ResolvedJavaType`）。
    fn get_array_class_resolved(&self) -> Box<dyn ResolvedJavaType>;

    /// 对应 `resolveMethod(ResolvedJavaMethod, ResolvedJavaType)`：返回 `Option` 对齐 Java nullable 语义。
    fn resolve_method(
        &self,
        method: &dyn ResolvedJavaMethod,
        caller_type: &dyn ResolvedJavaType,
    ) -> Option<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `resolveConcreteMethod(ResolvedJavaMethod, ResolvedJavaType)`（默认方法）：
    /// 返回 `Option` 对齐 Java nullable 语义。
    fn resolve_concrete_method(
        &self,
        method: &dyn ResolvedJavaMethod,
        caller_type: &dyn ResolvedJavaType,
    ) -> Option<Box<dyn ResolvedJavaMethod>> {
        let resolved_method = self.resolve_method(method, caller_type);
        match resolved_method {
            None => None,
            Some(m) => {
                if m.is_abstract() {
                    None
                } else {
                    Some(m)
                }
            }
        }
    }

    /// 对应 `findUniqueConcreteMethod(ResolvedJavaMethod)`。
    fn find_unique_concrete_method(
        &self,
        method: &dyn ResolvedJavaMethod,
    ) -> AssumptionResult<Option<Box<dyn ResolvedJavaMethod>>>;

    /// 对应 `getInstanceFields(boolean)`。
    fn get_instance_fields(&self, include_superclasses: bool) -> Vec<Box<dyn ResolvedJavaField>>;

    /// 对应 `getStaticFields()`。
    fn get_static_fields(&self) -> Vec<Box<dyn ResolvedJavaField>>;

    /// 对应 `findInstanceFieldWithOffset(long, JavaKind)`：返回 `Option` 对齐 Java nullable 语义。
    fn find_instance_field_with_offset(
        &self,
        offset: i64,
        expected_kind: JavaKind,
    ) -> Option<Box<dyn ResolvedJavaField>>;

    /// 对应 `getSourceFileName()`：返回 `Option` 对齐 Java nullable 语义（源文件名可缺）。
    fn get_source_file_name(&self) -> Option<String>;

    /// 对应 `isLocal()`。
    fn is_local(&self) -> bool;

    /// 对应 `isMember()`。
    fn is_member(&self) -> bool;

    /// 对应 `getEnclosingType()`：返回 `Option` 对齐 Java nullable 语义。
    fn get_enclosing_type(&self) -> Option<Box<dyn ResolvedJavaType>>;

    /// 对应 `getDeclaredConstructors()`。
    fn get_declared_constructors(&self) -> Vec<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `getDeclaredConstructors(boolean)`（默认抛 `UnsupportedOperationException`）。
    fn get_declared_constructors_with(
        &self,
        _force_link: bool,
    ) -> Vec<Box<dyn ResolvedJavaMethod>> {
        panic!("UnsupportedOperationException");
    }

    /// 对应 `getDeclaredMethods()`。
    fn get_declared_methods(&self) -> Vec<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `getDeclaredMethods(boolean)`（默认抛 `UnsupportedOperationException`）。
    fn get_declared_methods_with(&self, _force_link: bool) -> Vec<Box<dyn ResolvedJavaMethod>> {
        panic!("UnsupportedOperationException");
    }

    /// 对应 `getAllMethods(boolean)`。
    fn get_all_methods(&self, force_link: bool) -> Vec<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `getClassInitializer()`：返回 `Option` 对齐 Java nullable 语义。
    fn get_class_initializer(&self) -> Option<Box<dyn ResolvedJavaMethod>>;

    /// 对应 `findMethod(String, Signature)`（默认方法）：返回 `Option` 对齐 Java nullable 语义。
    ///
    /// 偏离记录：Java 用 `signature.equals(signature)`；`Signature` 无 `equals` 覆写，HotSpot
    /// 的 `HotSpotSignature.equals` 比较方法描述符字符串。Rust 侧以 `to_method_descriptor()`
    /// 字符串相等对齐 HotSpot 行为。
    fn find_method(
        &self,
        name: &str,
        signature: &dyn Signature,
    ) -> Option<Box<dyn ResolvedJavaMethod>> {
        let target_descriptor = signature.to_method_descriptor();
        self.get_declared_methods().into_iter().find(|method| {
            method.get_name() == name
                && method.get_signature().to_method_descriptor() == target_descriptor
        })
    }

    /// 对应 `isCloneableWithAllocation()`。
    fn is_cloneable_with_allocation(&self) -> bool;

    /// 对应 `lookupType(UnresolvedJavaType, boolean)`（默认返回 `None`，对齐 Java 返回 `null`）。
    fn lookup_type(
        &self,
        _unresolved_java_type: &UnresolvedJavaType,
        _resolve: bool,
    ) -> Option<Box<dyn ResolvedJavaType>> {
        None
    }

    /// 对应 `resolveField(UnresolvedJavaField, ResolvedJavaType)`（默认返回 `None`，对齐 Java 返回 `null`）。
    fn resolve_field(
        &self,
        _unresolved_java_field: &UnresolvedJavaField,
        _accessing_class: &dyn ResolvedJavaType,
    ) -> Option<Box<dyn ResolvedJavaField>> {
        None
    }

    /// 对应 `isConcrete()`（覆写 `ModifiersProvider.isConcrete` 默认方法）。
    fn is_concrete(&self) -> bool {
        self.is_array() || !self.is_abstract()
    }
}
