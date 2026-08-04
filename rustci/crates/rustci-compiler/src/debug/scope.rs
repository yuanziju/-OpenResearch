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
// Rust mirror of `jdk.graal.compiler.debug.Scope`.

use crate::debug::debug_closeable::DebugCloseable;
use crate::debug::indent::Indent;

/// Mirrors `jdk.graal.compiler.debug.Scope`.
/// A named debug scope that manages indentation and timing context.
/// Implements `DebugCloseable` (mirrors `AutoCloseable` in Java) for
/// try-with-resources semantics.
pub struct Scope {
    /// The name of this scope. Mirrors `Scope.name`.
    name: String,
    /// The parent scope, if any. Mirrors `Scope.parent`.
    parent: Option<Box<Scope>>,
    /// Reference to a shared indent. Mirrors the indentation management.
    indent: Option<Indent>,
    /// Whether this scope has been closed.
    closed: bool,
}

impl Scope {
    /// Creates a new root scope. Mirrors `Scope(DebugContext, String)`.
    pub fn new(name: String) -> Self {
        Scope {
            name,
            parent: None,
            indent: None,
            closed: false,
        }
    }

    /// Creates a child scope nested under a parent. Mirrors `Scope(DebugContext, String, Scope)`.
    pub fn child(name: String, parent: Scope) -> Self {
        Scope {
            name,
            parent: Some(Box::new(parent)),
            indent: None,
            closed: false,
        }
    }

    /// Creates a scope with an associated indent.
    pub fn with_indent(name: String, indent: Indent) -> Self {
        Scope {
            name,
            parent: None,
            indent: Some(indent),
            closed: false,
        }
    }

    /// Returns the name of this scope. Mirrors `Scope.getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the qualified name (dotted path from root). Mirrors `Scope.getQualifiedName()`.
    pub fn get_qualified_name(&self) -> String {
        let mut names = Vec::new();
        let mut current: Option<&Scope> = Some(self);
        while let Some(scope) = current {
            names.push(scope.name.clone());
            current = scope.parent.as_deref();
        }
        names.reverse();
        names.join(".")
    }

    /// Returns the parent scope, if any. Mirrors `Scope.getParent()`.
    pub fn get_parent(&self) -> Option<&Scope> {
        self.parent.as_deref()
    }

    /// Returns whether the scope has been closed. Mirrors the closed state.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Returns the indent reference.
    pub fn get_indent(&self) -> Option<&Indent> {
        self.indent.as_ref()
    }

    /// Increments indentation if this scope has an indent.
    pub fn indent(&self) {
        if let Some(ref indent) = self.indent {
            indent.indent();
        }
    }
}

impl DebugCloseable for Scope {
    fn close(&mut self) {
        if !self.closed {
            self.closed = true;
            if let Some(ref indent) = self.indent {
                indent.outdent();
            }
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        if !self.closed {
            if let Some(ref indent) = self.indent {
                indent.outdent();
            }
        }
    }
}