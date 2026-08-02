// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2011, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.code.MemoryBarriers`：内存屏障常量与工具。

/// `LoadLoad` 屏障位。
pub const LOAD_LOAD: i32 = 0x0001;
/// `LoadStore` 屏障位。
pub const LOAD_STORE: i32 = 0x0002;
/// `StoreLoad` 屏障位。
pub const STORE_LOAD: i32 = 0x0004;
/// `StoreStore` 屏障位。
pub const STORE_STORE: i32 = 0x0008;

pub const JMM_PRE_VOLATILE_WRITE: i32 = LOAD_STORE | STORE_STORE;
pub const JMM_POST_VOLATILE_WRITE: i32 = STORE_LOAD | STORE_STORE;
pub const JMM_PRE_VOLATILE_READ: i32 = 0;
pub const JMM_POST_VOLATILE_READ: i32 = LOAD_LOAD | LOAD_STORE;

/// 对应 `barriersString(int)`。
pub fn barriers_string(barriers: i32) -> String {
    let mut sb = String::new();
    if (barriers & LOAD_LOAD) != 0 {
        sb.push_str("LOAD_LOAD ");
    }
    if (barriers & LOAD_STORE) != 0 {
        sb.push_str("LOAD_STORE ");
    }
    if (barriers & STORE_LOAD) != 0 {
        sb.push_str("STORE_LOAD ");
    }
    if (barriers & STORE_STORE) != 0 {
        sb.push_str("STORE_STORE ");
    }
    sb.trim_end().to_string()
}
