/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

use std::io;

/// Thrown from BinaryReader when a BGV file's header is missing or reports an
/// unsupported format version.
#[derive(Debug)]
pub struct VersionMismatchException {
    message: String,
}

impl VersionMismatchException {
    pub fn new(message: String) -> Self {
        VersionMismatchException { message }
    }
}

impl std::fmt::Display for VersionMismatchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VersionMismatchException {}

impl From<VersionMismatchException> for io::Error {
    fn from(e: VersionMismatchException) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e.message)
    }
}
