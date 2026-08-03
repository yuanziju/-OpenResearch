/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Provides source location information about compiled code. This is an extension
/// of `GraphElements` - by default the elements work with classical
/// `StackTraceElement`. Should the default behavior not be sufficient, implement
/// this trait to provide additional operations.
pub trait GraphLocations<M, P, L> {
    /// Returns all applicable source locations for the given code position.
    fn method_location(&self, method: &M, bci: i32, pos: &P) -> Vec<L>;

    /// Identification of the language/stratum for a location.
    fn location_language(&self, location: &L) -> Option<String>;

    /// The URI that contains the location.
    fn location_uri(&self, location: &L) -> Option<String>;

    /// Line number of a location. Negative means not available.
    fn location_line_number(&self, location: &L) -> i32;

    /// Starting offset of the location. Negative means not available.
    fn location_offset_start(&self, location: &L) -> i32;

    /// End offset (exclusive) of the location. Negative means not available.
    fn location_offset_end(&self, location: &L) -> i32;
}
