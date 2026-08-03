/*
 * Copyright (c) 2011, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 * ...
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception

/// Representation of methods, fields, their signatures, and code locations.
pub trait GraphElements<M, F, S, P> {
    /// Recognize a method. Returns `None` if the object is not a method.
    fn method(&self, obj: &dyn std::any::Any) -> Option<M>;

    /// Bytecode for a method.
    fn method_code(&self, method: &M) -> Vec<u8>;

    /// Method modifiers.
    fn method_modifiers(&self, method: &M) -> i32;

    /// Method's signature.
    fn method_signature(&self, method: &M) -> S;

    /// Method name.
    fn method_name(&self, method: &M) -> String;

    /// Method's declaring class.
    fn method_declaring_class(&self, method: &M) -> Box<dyn std::any::Any>;

    /// Recognize a field. Returns `None` if the object is not a field.
    fn field(&self, object: &dyn std::any::Any) -> Option<F>;

    /// Field modifiers.
    fn field_modifiers(&self, field: &F) -> i32;

    /// Type name of the field.
    fn field_type_name(&self, field: &F) -> String;

    /// Name of a field.
    fn field_name(&self, field: &F) -> String;

    /// Field's declaring class.
    fn field_declaring_class(&self, field: &F) -> Box<dyn std::any::Any>;

    /// Recognize a signature. Returns `None` if the object is not a signature.
    fn signature(&self, object: &dyn std::any::Any) -> Option<S>;

    /// Number of parameters of a signature.
    fn signature_parameter_count(&self, signature: &S) -> usize;

    /// Type name of a signature parameter.
    fn signature_parameter_type_name(&self, signature: &S, index: usize) -> String;

    /// Type name of a return type.
    fn signature_return_type_name(&self, signature: &S) -> String;

    /// Recognize a source position. Returns `None` if the object is not a position.
    fn node_source_position(&self, object: &dyn std::any::Any) -> Option<P>;

    /// Method for a position.
    fn node_source_position_method(&self, pos: &P) -> M;

    /// Caller of a position.
    fn node_source_position_caller(&self, pos: &P) -> Option<P>;

    /// Byte code index of a position.
    fn node_source_position_bci(&self, pos: &P) -> i32;

    /// Stack trace element for a method, bci and position.
    fn method_stack_trace_element(&self, method: &M, bci: i32, pos: &P) -> StackTraceElement;
}

/// A simplified stack trace element for Java-like languages.
#[derive(Debug, Clone)]
pub struct StackTraceElement {
    pub class_name: String,
    pub method_name: String,
    pub file_name: Option<String>,
    pub line_number: i32,
}

impl StackTraceElement {
    pub fn new(
        class_name: String,
        method_name: String,
        file_name: Option<String>,
        line_number: i32,
    ) -> Self {
        StackTraceElement {
            class_name,
            method_name,
            file_name,
            line_number,
        }
    }
}
