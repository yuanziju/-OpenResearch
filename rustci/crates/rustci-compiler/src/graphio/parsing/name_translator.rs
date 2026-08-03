/*
 * Copyright (c) 2013, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Used by BinaryReader to translate class names read from a DataSource,
/// e.g. according to a translation file.
pub trait NameTranslator {
    fn translate(&self, fqn: &str) -> String;
}
