/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Special support for dealing with enums. When real `Enum` instances cannot be
/// used, an implementation of this trait can be registered with the Builder to
/// treat enum-like values specially.
pub trait GraphTypes {
    /// Recognizes an "enum" object. Returns the class of the enum value, or
    /// `None` if the value is not an enum.
    fn enum_class(&self, enum_value: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>>;

    /// Ordinal of an enum. Returns `-1` if the object is not an enum.
    fn enum_ordinal(&self, obj: &dyn std::any::Any) -> i32;

    /// All possible values of an enum. Returns the names of enum values in
    /// ordinal order, or `None` if the class is not an enum.
    fn enum_type_values(&self, maybe_enum_class: &dyn std::any::Any) -> Option<Vec<String>>;

    /// Finds the Java type name for a given class.
    fn type_name(&self, maybe_class: &dyn std::any::Any) -> Option<String>;
}
