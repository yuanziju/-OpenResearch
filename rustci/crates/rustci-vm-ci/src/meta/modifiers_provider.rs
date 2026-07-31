// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2014, 2024, Oracle and/or its affiliates. All rights reserved.
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

//! 镜像 `jdk.vm.ci.meta.ModifiersProvider` 与 `java.lang.reflect.Modifier` 的位常量/判定函数。

/// `public` 修饰符位。
pub const PUBLIC: i32 = 0x0001;
/// `private` 修饰符位。
pub const PRIVATE: i32 = 0x0002;
/// `protected` 修饰符位。
pub const PROTECTED: i32 = 0x0004;
/// `static` 修饰符位。
pub const STATIC: i32 = 0x0008;
/// `final` 修饰符位。
pub const FINAL: i32 = 0x0010;
/// `synchronized` 修饰符位。
pub const SYNCHRONIZED: i32 = 0x0020;
/// `volatile` 修饰符位。
pub const VOLATILE: i32 = 0x0040;
/// `transient` 修饰符位。
pub const TRANSIENT: i32 = 0x0080;
/// `native` 修饰符位。
pub const NATIVE: i32 = 0x0100;
/// `interface` 修饰符位。
pub const INTERFACE: i32 = 0x0200;
/// `abstract` 修饰符位。
pub const ABSTRACT: i32 = 0x0400;
/// `strictfp` 修饰符位。
pub const STRICT: i32 = 0x0800;

pub fn is_interface(modifiers: i32) -> bool {
    (modifiers & INTERFACE) != 0
}
pub fn is_synchronized(modifiers: i32) -> bool {
    (modifiers & SYNCHRONIZED) != 0
}
pub fn is_static(modifiers: i32) -> bool {
    (modifiers & STATIC) != 0
}
pub fn is_final(modifiers: i32) -> bool {
    (modifiers & FINAL) != 0
}
pub fn is_public(modifiers: i32) -> bool {
    (modifiers & PUBLIC) != 0
}
pub fn is_private(modifiers: i32) -> bool {
    (modifiers & PRIVATE) != 0
}
pub fn is_protected(modifiers: i32) -> bool {
    (modifiers & PROTECTED) != 0
}
pub fn is_transient(modifiers: i32) -> bool {
    (modifiers & TRANSIENT) != 0
}
pub fn is_strict(modifiers: i32) -> bool {
    (modifiers & STRICT) != 0
}
pub fn is_volatile(modifiers: i32) -> bool {
    (modifiers & VOLATILE) != 0
}
pub fn is_native(modifiers: i32) -> bool {
    (modifiers & NATIVE) != 0
}
pub fn is_abstract(modifiers: i32) -> bool {
    (modifiers & ABSTRACT) != 0
}

/// 对应 `interface ModifiersProvider`：由一组 Java 语言修饰符描述的元素。
pub trait ModifiersProvider {
    fn get_modifiers(&self) -> i32;

    fn is_interface(&self) -> bool {
        is_interface(self.get_modifiers())
    }
    fn is_synchronized(&self) -> bool {
        is_synchronized(self.get_modifiers())
    }
    fn is_static(&self) -> bool {
        is_static(self.get_modifiers())
    }
    /// 对应 `isFinalFlagSet`（不对外导出为 `isFinal`，与 Java 一致：类型的 final 位语义混乱）。
    fn is_final_flag_set(&self) -> bool {
        is_final(self.get_modifiers())
    }
    fn is_public(&self) -> bool {
        is_public(self.get_modifiers())
    }
    fn is_package_private(&self) -> bool {
        (PUBLIC | PROTECTED | PRIVATE) & self.get_modifiers() == 0
    }
    fn is_private(&self) -> bool {
        is_private(self.get_modifiers())
    }
    fn is_protected(&self) -> bool {
        is_protected(self.get_modifiers())
    }
    fn is_transient(&self) -> bool {
        is_transient(self.get_modifiers())
    }
    fn is_strict(&self) -> bool {
        is_strict(self.get_modifiers())
    }
    fn is_volatile(&self) -> bool {
        is_volatile(self.get_modifiers())
    }
    fn is_native(&self) -> bool {
        is_native(self.get_modifiers())
    }
    fn is_abstract(&self) -> bool {
        is_abstract(self.get_modifiers())
    }
    /// 对应 `isConcrete`：方法有具体实现或类型可实例化。
    fn is_concrete(&self) -> bool {
        !self.is_abstract()
    }
}
