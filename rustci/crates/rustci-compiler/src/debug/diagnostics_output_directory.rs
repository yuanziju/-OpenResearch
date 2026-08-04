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
// Rust mirror of `jdk.graal.compiler.debug.DiagnosticsOutputDirectory`.

use std::path::PathBuf;

/// Mirrors `jdk.graal.compiler.debug.DiagnosticsOutputDirectory`.
/// Manages the directory for diagnostic output files (dumps, logs, etc.).
#[derive(Debug, Clone)]
pub struct DiagnosticsOutputDirectory {
    /// The path to the output directory. Mirrors `DiagnosticsOutputDirectory.outputDirectory`.
    output_directory: PathBuf,
}

impl DiagnosticsOutputDirectory {
    /// Creates a new output directory reference. Mirrors
    /// `DiagnosticsOutputDirectory(Path)`.
    pub fn new(path: PathBuf) -> Self {
        DiagnosticsOutputDirectory {
            output_directory: path,
        }
    }

    /// Returns the output directory path. Mirrors
    /// `DiagnosticsOutputDirectory.getOutputDirectory()`.
    pub fn get_output_directory(&self) -> &PathBuf {
        &self.output_directory
    }

    /// Resolves a file name within the output directory.
    /// Mirrors `DiagnosticsOutputDirectory.resolve(String)`.
    pub fn resolve(&self, name: &str) -> PathBuf {
        self.output_directory.join(name)
    }

    /// Creates a file path for a dump with a specific id and name.
    /// Mirrors the pattern in `DiagnosticsOutputDirectory.createDumpFile`.
    pub fn create_dump_file(&self, id: &str, name: &str, extension: &str) -> PathBuf {
        let filename = format!("{}_{}.{}", id, name, extension);
        self.resolve(&filename)
    }

    /// Returns the string representation of the directory path.
    /// Mirrors `DiagnosticsOutputDirectory.toString()`.
    pub fn to_string(&self) -> String {
        self.output_directory.display().to_string()
    }
}