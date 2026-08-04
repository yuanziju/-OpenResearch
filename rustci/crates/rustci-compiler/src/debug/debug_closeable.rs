/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
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

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.debug.DebugCloseable` (mirrors `java.lang.AutoCloseable`).

/// Mirrors `java.lang.AutoCloseable` as used by `jdk.graal.compiler.debug.DebugCloseable`.
/// Objects that implement this trait can be used in try-with-resources style patterns.
pub trait DebugCloseable {
    /// Closes this resource, relinquishing any underlying resources.
    /// Mirrors `AutoCloseable.close()`.
    fn close(&mut self);
}

/// A no-op implementation of `DebugCloseable`, mirroring the pattern of
/// returning a dummy closeable when debugging is disabled.
pub struct NoopCloseable;

impl DebugCloseable for NoopCloseable {
    fn close(&mut self) {}
}

/// A wrapper that calls `close` on drop, mirroring try-with-resources semantics.
pub struct DebugCloseableGuard<T: DebugCloseable> {
    inner: Option<T>,
}

impl<T: DebugCloseable> DebugCloseableGuard<T> {
    pub fn new(inner: T) -> Self {
        DebugCloseableGuard { inner: Some(inner) }
    }
}

impl<T: DebugCloseable> Drop for DebugCloseableGuard<T> {
    fn drop(&mut self) {
        if let Some(ref mut inner) = self.inner {
            inner.close();
        }
    }
}