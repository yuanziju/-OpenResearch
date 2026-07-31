// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.UnresolvedJavaMethod`：未解析方法的占位实现。
//!
//! 偏离记录：
//! - Java `final class UnresolvedJavaMethod implements JavaMethod` → Rust `pub struct
//!   UnresolvedJavaMethod` 实现 `JavaMethod` trait。
//! - Java `Throwable cause`（可空）→ Rust `Option<Box<dyn Any + Send + Sync>>`（对齐
//!   Java `Throwable` 的不透明语义；T7 绑定具体错误类型时替换，同 `UnresolvedJavaType`）。
//! - Java `equals` 调 `name.equals`/`signature.equals`/`holder.equals`；`Signature` 无
//!   `equals` 覆写（HotSpot `HotSpotSignature.equals` 比较方法描述符字符串），Rust 侧以
//!   `to_method_descriptor()` 字符串相等对齐（同 `ResolvedJavaType.findMethod` 偏离）。
//!   `holder` 为 `JavaType` trait 对象，多态 `equals` 退化为指针相等（同 `Local`/
//!   `ExceptionHandler` 偏离）。
//! - Java `hashCode` 返回 `super.hashCode()`（identity）→ Rust 用指针地址哈希占位
//!   （同 `Local` 偏离；T7 用共享指针地址）。
//! - Java 无 `toString` 覆写（继承 `Object.toString` = `类名@十六进制哈希`）；Rust 侧
//!   `Display`/`Debug` 以 `"UnresolvedJavaMethod@" + 指针地址` 近似（偏离：输出指针地址
//!   而非 identity hash，T7 用共享指针地址对齐）。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_method::JavaMethod;
use crate::meta::java_type::JavaType;
use crate::meta::signature::Signature;

/// 对应 `public final class UnresolvedJavaMethod implements JavaMethod`。
pub struct UnresolvedJavaMethod {
    name: String,
    signature: Box<dyn Signature>,
    holder: Box<dyn JavaType>,
    /// 对应 Java `Throwable cause`（可空）。
    cause: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl UnresolvedJavaMethod {
    /// 对应 `UnresolvedJavaMethod(String name, Signature signature, JavaType holder, Throwable cause)`。
    pub fn new(
        name: impl Into<String>,
        signature: Box<dyn Signature>,
        holder: Box<dyn JavaType>,
        cause: Option<Box<dyn std::any::Any + Send + Sync>>,
    ) -> Self {
        Self {
            name: name.into(),
            signature,
            holder,
            cause,
        }
    }

    /// 对应 `UnresolvedJavaMethod(String name, Signature signature, JavaType holder)`：cause 为 null。
    pub fn without_cause(
        name: impl Into<String>,
        signature: Box<dyn Signature>,
        holder: Box<dyn JavaType>,
    ) -> Self {
        Self::new(name, signature, holder, None)
    }

    /// 对应 `Throwable getCause()`：返回 `Option` 对齐 Java nullable。
    pub fn get_cause(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
        self.cause.as_deref()
    }
}

impl JavaMethod for UnresolvedJavaMethod {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_declaring_class(&self) -> &dyn JavaType {
        self.holder.as_ref()
    }

    fn get_signature(&self) -> &dyn Signature {
        self.signature.as_ref()
    }
}

impl PartialEq for UnresolvedJavaMethod {
    fn eq(&self, other: &Self) -> bool {
        // 对应 Java `equals`：name 字符串相等 + signature 描述符相等 + holder 引用相等
        // （偏离：holder 多态 equals 退化为指针相等）。
        self.name == other.name
            && self.signature.to_method_descriptor() == other.signature.to_method_descriptor()
            && std::ptr::eq(&*self.holder, &*other.holder)
    }
}

impl Eq for UnresolvedJavaMethod {}

impl Hash for UnresolvedJavaMethod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 对应 `super.hashCode()`（identity hash）；Rust 用指针地址占位。
        let addr = self as *const Self as usize;
        (addr as u64).hash(state);
    }
}

impl fmt::Debug for UnresolvedJavaMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnresolvedJavaMethod@{:x}", self as *const Self as usize)
    }
}

impl fmt::Display for UnresolvedJavaMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnresolvedJavaMethod@{:x}", self as *const Self as usize)
    }
}
