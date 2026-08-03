// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2016, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.hotspot.HotSpotMemoryAccessProvider`：HotSpot 内存访问提供者。
//!
//! 偏离记录：
//! - Java `interface HotSpotMemoryAccessProvider extends MemoryAccessProvider` →
//!   Rust `pub trait HotSpotMemoryAccessProvider: MemoryAccessProvider`。
//! - 本期只移植接口方法签名，具体实现留待后端绑定。
//! - 方法名 camelCase → snake_case。

use crate::meta::constant::Constant;
use crate::meta::java_constant::JavaConstant;
use crate::meta::memory_access::MemoryAccessProvider;

/// 对应 `interface HotSpotMemoryAccessProvider extends MemoryAccessProvider`。
pub trait HotSpotMemoryAccessProvider: MemoryAccessProvider {
    /// 对应 `JavaConstant readNarrowOopConstant(Constant, long)`。
    fn read_narrow_oop_constant(
        &self,
        base: &dyn Constant,
        displacement: i64,
    ) -> Box<dyn JavaConstant>;

    /// 对应 `Constant readKlassPointerConstant(Constant, long)`。
    fn read_klass_pointer_constant(
        &self,
        base: &dyn Constant,
        displacement: i64,
    ) -> Box<dyn Constant>;

    /// 对应 `Constant readNarrowKlassPointerConstant(Constant, long)`。
    fn read_narrow_klass_pointer_constant(
        &self,
        base: &dyn Constant,
        displacement: i64,
    ) -> Box<dyn Constant>;

    /// 对应 `Constant readMethodPointerConstant(Constant, long)`。
    fn read_method_pointer_constant(
        &self,
        base: &dyn Constant,
        displacement: i64,
    ) -> Box<dyn Constant>;
}
