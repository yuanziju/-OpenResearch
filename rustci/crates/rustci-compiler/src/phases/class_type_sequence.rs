/*
 * Copyright (c) 2011, 2025, Oracle and/or its affiliates. All rights reserved.
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
// Rust mirror of `jdk.graal.compiler.phases.ClassTypeSequence`.

/// A printable representation of the name of a class that can be serialized
/// as a fully qualified type for dumping.
/// Mirrors `jdk.graal.compiler.phases.ClassTypeSequence`.
#[derive(Debug, Clone)]
pub struct ClassTypeSequence {
    /// The fully qualified class name (e.g., "com.example.MyClass").
    class_name: String,
    /// Cached simple name (the part after the last dot).
    simple_name: Option<String>,
}

impl ClassTypeSequence {
    /// Creates a new ClassTypeSequence from a class name string.
    pub fn new(class_name: String) -> Self {
        Self {
            class_name,
            simple_name: None,
        }
    }

    /// Returns the internal JVM name (e.g., "Lcom/example/MyClass;").
    pub fn get_name(&self) -> String {
        format!("L{};", self.class_name.replace('.', "/"))
    }

    /// Returns the Java name, optionally qualified.
    pub fn to_java_name(&self, qualified: bool) -> String {
        if qualified {
            self.class_name.clone()
        } else {
            if let Some(ref name) = self.simple_name {
                return name.clone();
            }
            let name = self
                .class_name
                .rfind('.')
                .map(|idx| self.class_name[idx + 1..].to_string())
                .unwrap_or_else(|| self.class_name.clone());
            name
        }
    }

    /// Returns the length of the class name.
    pub fn len(&self) -> usize {
        self.class_name.len()
    }

    /// Returns true if the class name is empty.
    pub fn is_empty(&self) -> bool {
        self.class_name.is_empty()
    }

    /// Returns the character at the given index.
    pub fn char_at(&self, index: usize) -> char {
        self.class_name.chars().nth(index).unwrap_or('\0')
    }
}

impl std::fmt::Display for ClassTypeSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_java_name(false))
    }
}