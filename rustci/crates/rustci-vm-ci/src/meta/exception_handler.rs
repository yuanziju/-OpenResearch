// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2009, 2012, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ExceptionHandler`：字节码异常处理器表项。
//!
//! 偏离记录：Java `final class ExceptionHandler` 持 `JavaType catchType`（可空——`catchTypeCPI == 0`
//! 即 catch-all），Rust 侧持 `Box<dyn JavaType>`，构造期保证非空（catch-all 由调用方传
//! `UnresolvedJavaType` 或具体类型表达）。Java `equals` 中 `Objects.equals(catchType, catchType)`
//! → Rust `catchType` 指针相等（trait 对象无多态 equals，对齐 HotSpot 引用相等）。`hashCode`
//! 返回 `catchTypeCPI ^ endBCI ^ handlerBCI`。

use std::fmt;
use std::hash::{Hash, Hasher};

use crate::meta::java_type::JavaType;

/// 对应 `public final class ExceptionHandler`。
pub struct ExceptionHandler {
    start_bci: i32,
    end_bci: i32,
    handler_bci: i32,
    catch_type_cpi: i32,
    catch_type: Box<dyn JavaType>,
}

impl ExceptionHandler {
    /// 对应 `ExceptionHandler(int startBCI, int endBCI, int catchBCI, int catchTypeCPI, JavaType catchType)`。
    pub fn new(
        start_bci: i32,
        end_bci: i32,
        catch_bci: i32,
        catch_type_cpi: i32,
        catch_type: Box<dyn JavaType>,
    ) -> Self {
        Self {
            start_bci,
            end_bci,
            handler_bci: catch_bci,
            catch_type_cpi,
            catch_type,
        }
    }

    /// 对应 `getStartBCI()`。
    pub fn get_start_bci(&self) -> i32 {
        self.start_bci
    }

    /// 对应 `getEndBCI()`。
    pub fn get_end_bci(&self) -> i32 {
        self.end_bci
    }

    /// 对应 `getHandlerBCI()`。
    pub fn get_handler_bci(&self) -> i32 {
        self.handler_bci
    }

    /// 对应 `catchTypeCPI()`。
    pub fn catch_type_cpi(&self) -> i32 {
        self.catch_type_cpi
    }

    /// 对应 `isCatchAll()`。
    pub fn is_catch_all(&self) -> bool {
        self.catch_type_cpi == 0
    }

    /// 对应 `getCatchType()`：返回 `&dyn JavaType`（Java 返回 `JavaType`）。
    pub fn get_catch_type(&self) -> &dyn JavaType {
        self.catch_type.as_ref()
    }
}

impl fmt::Debug for ExceptionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExceptionHandler")
            .field("startBCI", &self.start_bci)
            .field("endBCI", &self.end_bci)
            .field("handlerBCI", &self.handler_bci)
            .field("catchTypeCPI", &self.catch_type_cpi)
            .field("catchType", &self.catch_type)
            .finish()
    }
}

impl fmt::Display for ExceptionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 对应 `toString()`。Java `catchType` 经 `%s` 调 `toString()`（`JavaType` 无覆写即
        // `Object.toString`）；Rust 侧 `dyn JavaType` 无 `Display`，改用 `to_java_name()`
        // 作可读近似（偏离：输出 Java 名而非 `type@hash`）。
        write!(
            f,
            "ExceptionHandler<startBCI={}, endBCI={}, handlerBCI={}, catchTypeCPI={}, catchType={}>",
            self.start_bci, self.end_bci, self.handler_bci, self.catch_type_cpi, self.catch_type.to_java_name()
        )
    }
}

impl PartialEq for ExceptionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.start_bci == other.start_bci
            && self.end_bci == other.end_bci
            && self.handler_bci == other.handler_bci
            && self.catch_type_cpi == other.catch_type_cpi
            && std::ptr::eq(&*self.catch_type, &*other.catch_type)
    }
}

impl Eq for ExceptionHandler {}

impl Hash for ExceptionHandler {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 对应 `hashCode() { return catchTypeCPI ^ endBCI ^ handlerBCI; }`
        (self.catch_type_cpi ^ self.end_bci ^ self.handler_bci).hash(state);
    }
}
