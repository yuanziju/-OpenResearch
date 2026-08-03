/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * The Universal Permissive License (UPL), Version 1.0
 *
 * Subject to the condition set forth below, permission is hereby granted to any
 * person obtaining a copy of this software, associated documentation and/or
 * data (collectively the "Software"), free of charge and under any and all
 * copyright rights in the Software, and any and all patent rights owned or
 * freely licensable by each licensor hereunder covering either (i) the
 * unmodified Software as contributed to or provided by such licensor, or (ii)
 * the Larger Works (as defined below), to deal in both
 *
 * (a) the Software, and
 *
 * (b) any piece of software and/or hardware listed in the lrgrwrks.txt file if
 * one is included with the Software each a "Larger Work" to which the Software
 * is contributed by such licensors),
 *
 * without restriction, including without limitation the rights to copy, create
 * derivative works of, display, perform, and distribute the Software and make,
 * use, sell, offer for sale, import, export, have made, and have sold the
 * Software and the Larger Work(s), and to sublicense the foregoing rights on
 * either these or other terms.
 *
 * This license is subject to the following condition:
 *
 * The above copyright notice and either this complete permission notice or at a
 * minimum a reference to the UPL must be included in all copies or substantial
 * portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Port of `jdk.graal.compiler.util.json.JsonBuilder`.
//
// Java uses nested inner classes (`ExclusiveBuilder`, `ObjectBuilder`,
// `ArrayBuilder`, `ValueBuilder`) that share the enclosing `JsonBuilder`'s
// `JsonWriter` and a `currentBuilder` field for exclusive-access
// enforcement.  Rust uses `Rc<RefCell<Box<dyn JsonWriter>>>` for shared
// mutable writer access and `Rc<Cell<u64>>` for active-builder tracking.

use crate::json_value::JsonValue;
use crate::json_writer_trait::JsonWriter;
use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;

// ── ExclusiveState ───────────────────────────────────────────────────────

/// Shared state for exclusive access tracking. Mirrors the Java
/// `ExclusiveBuilder` + `JsonBuilder.currentBuilder` pattern.
struct ExclusiveState {
    writer: Rc<RefCell<Box<dyn JsonWriter>>>,
    active_id: Rc<Cell<u64>>,
    next_id: Rc<Cell<u64>>,
    id: u64,
    parent_id: u64,
    closed: Cell<bool>,
}

impl ExclusiveState {
    fn new(
        writer: Rc<RefCell<Box<dyn JsonWriter>>>,
        active_id: Rc<Cell<u64>>,
        next_id: Rc<Cell<u64>>,
        parent_id: u64,
    ) -> Self {
        let id = next_id.get();
        next_id.set(id + 1);
        active_id.set(id);
        ExclusiveState {
            writer,
            active_id,
            next_id,
            id,
            parent_id,
            closed: Cell::new(false),
        }
    }

    /// Mirrors `ExclusiveBuilder.checkAccess()`.
    fn check_access(&self) -> io::Result<()> {
        if self.closed.get() {
            return Err(io::Error::other("Builder instance is already closed"));
        }
        if self.active_id.get() != self.id {
            return Err(io::Error::other(
                "Builder instance is not currently responsible for printing",
            ));
        }
        Ok(())
    }

    /// Borrows the writer mutably.
    fn writer_mut(&self) -> std::cell::RefMut<'_, Box<dyn JsonWriter>> {
        self.writer.borrow_mut()
    }

    /// Mirrors `ExclusiveBuilder.finish()`. Transfers access back to the
    /// parent and marks this builder as closed.
    fn finish(&self) -> io::Result<()> {
        self.check_access()?;
        self.active_id.set(self.parent_id);
        self.closed.set(true);
        Ok(())
    }
}

// ── JsonBuilder (top-level factory) ──────────────────────────────────────

/// Mirrors `jdk.graal.compiler.util.json.JsonBuilder`.
///
/// Factory for creating nested builders (`ObjectBuilder`, `ArrayBuilder`,
/// `ValueBuilder`) that write well-structured JSON to an underlying
/// `JsonWriter`.
pub struct JsonBuilder {
    writer: Rc<RefCell<Box<dyn JsonWriter>>>,
    active_id: Rc<Cell<u64>>,
    next_id: Rc<Cell<u64>>,
}

impl JsonBuilder {
    /// Creates a new `JsonBuilder` with the given writer. Mirrors the
    /// private `JsonBuilder(JsonWriter)` constructor.
    fn new(writer: Box<dyn JsonWriter>) -> Self {
        let writer = Rc::new(RefCell::new(writer));
        JsonBuilder {
            writer,
            active_id: Rc::new(Cell::new(0)),
            next_id: Rc::new(Cell::new(1)),
        }
    }

    /// Start building a JSON object. Mirrors `JsonBuilder.object(JsonWriter)`.
    pub fn object(writer: Box<dyn JsonWriter>) -> io::Result<ObjectBuilder> {
        let builder = JsonBuilder::new(writer);
        builder.into_object()
    }

    /// Start building a JSON array. Mirrors `JsonBuilder.array(JsonWriter)`.
    pub fn array(writer: Box<dyn JsonWriter>) -> io::Result<ArrayBuilder> {
        let builder = JsonBuilder::new(writer);
        builder.into_array()
    }

    /// Start building an arbitrary JSON value. Mirrors
    /// `JsonBuilder.value(JsonWriter)`.
    pub fn value(writer: Box<dyn JsonWriter>) -> ValueBuilder {
        let builder = JsonBuilder::new(writer);
        builder.into_value()
    }

    fn into_object(self) -> io::Result<ObjectBuilder> {
        self.into_value().object()
    }

    fn into_array(self) -> io::Result<ArrayBuilder> {
        self.into_value().array()
    }

    fn into_value(self) -> ValueBuilder {
        ValueBuilder::new(
            self.writer,
            self.active_id,
            self.next_id,
            0, // parent_id = 0 (root)
        )
    }
}

// ── ObjectBuilder ────────────────────────────────────────────────────────

/// Builds a single well-formed JSON object. Mirrors
/// `JsonBuilder.ObjectBuilder`.
///
/// Implements `Drop` to match Java's `AutoCloseable` (calls `finish` on
/// drop if not already closed).
pub struct ObjectBuilder {
    state: ExclusiveState,
    is_first: bool,
}

impl ObjectBuilder {
    fn new(
        writer: Rc<RefCell<Box<dyn JsonWriter>>>,
        active_id: Rc<Cell<u64>>,
        next_id: Rc<Cell<u64>>,
        parent_id: u64,
    ) -> io::Result<Self> {
        let builder = ObjectBuilder {
            state: ExclusiveState::new(writer, active_id, next_id, parent_id),
            is_first: true,
        };
        builder.state.writer_mut().append_object_start()?;
        Ok(builder)
    }

    /// Writes a constant key and constant value into the object. Mirrors
    /// `ObjectBuilder.append(String, Object)`.
    pub fn append(mut self, key: &str, value: &JsonValue) -> io::Result<Self> {
        self.append_key(key)?;
        self.state.writer_mut().print(value)?;
        Ok(self)
    }

    /// Writes the given key into the object and returns a `ValueBuilder`
    /// responsible for producing the key's value. Mirrors
    /// `ObjectBuilder.append(String)`.
    pub fn append_key(&mut self, key: &str) -> io::Result<ValueBuilder> {
        self.state.check_access()?;
        if !self.is_first {
            self.state.writer_mut().append_separator()?;
        } else {
            self.is_first = false;
        }
        self.state.writer_mut().quote(key)?;
        self.state.writer_mut().append_field_separator()?;
        Ok(ValueBuilder::new(
            self.state.writer.clone(),
            self.state.active_id.clone(),
            self.state.next_id.clone(),
            self.state.id,
        ))
    }

    /// Finishes writing the object. Mirrors `ObjectBuilder.finish()`.
    pub fn finish(self) -> io::Result<()> {
        self.state.writer_mut().append_object_end()?;
        self.state.finish()
    }
}

impl Drop for ObjectBuilder {
    fn drop(&mut self) {
        if !self.state.closed.get() {
            let _ = self.state.writer_mut().append_object_end();
            let _ = self.state.finish();
        }
    }
}

// ── ArrayBuilder ─────────────────────────────────────────────────────────

/// Builds a single well-formed JSON array. Mirrors `JsonBuilder.ArrayBuilder`.
///
/// Implements `Drop` to match Java's `AutoCloseable`.
pub struct ArrayBuilder {
    state: ExclusiveState,
    is_first: bool,
}

impl ArrayBuilder {
    fn new(
        writer: Rc<RefCell<Box<dyn JsonWriter>>>,
        active_id: Rc<Cell<u64>>,
        next_id: Rc<Cell<u64>>,
        parent_id: u64,
    ) -> io::Result<Self> {
        let builder = ArrayBuilder {
            state: ExclusiveState::new(writer, active_id, next_id, parent_id),
            is_first: true,
        };
        builder.state.writer_mut().append_array_start()?;
        Ok(builder)
    }

    /// Appends a constant value to the array. Mirrors
    /// `ArrayBuilder.append(Object)`.
    pub fn append(mut self, value: &JsonValue) -> io::Result<Self> {
        self.next_entry_prepare()?;
        self.state.writer_mut().print(value)?;
        Ok(self)
    }

    /// Prepares the array for a new element and returns a `ValueBuilder`
    /// responsible to produce that new element. Mirrors
    /// `ArrayBuilder.nextEntry()`.
    pub fn next_entry(mut self) -> io::Result<ValueBuilder> {
        self.next_entry_prepare()?;
        Ok(ValueBuilder::new(
            self.state.writer.clone(),
            self.state.active_id.clone(),
            self.state.next_id.clone(),
            self.state.id,
        ))
    }

    fn next_entry_prepare(&mut self) -> io::Result<()> {
        self.state.check_access()?;
        if !self.is_first {
            self.state.writer_mut().append_separator()?;
        } else {
            self.is_first = false;
        }
        Ok(())
    }

    /// Finishes writing the array. Mirrors `ArrayBuilder.finish()`.
    pub fn finish(self) -> io::Result<()> {
        self.state.writer_mut().append_array_end()?;
        self.state.finish()
    }
}

impl Drop for ArrayBuilder {
    fn drop(&mut self) {
        if !self.state.closed.get() {
            let _ = self.state.writer_mut().append_array_end();
            let _ = self.state.finish();
        }
    }
}

// ── ValueBuilder ─────────────────────────────────────────────────────────

/// Builder responsible for writing exactly one JSON value. Mirrors
/// `JsonBuilder.ValueBuilder`.
///
/// Instance does not have to be closed manually. It performs clean-up
/// automatically when it has produced a complete JSON value.
pub struct ValueBuilder {
    state: ExclusiveState,
    wrote_something: bool,
}

impl ValueBuilder {
    fn new(
        writer: Rc<RefCell<Box<dyn JsonWriter>>>,
        active_id: Rc<Cell<u64>>,
        next_id: Rc<Cell<u64>>,
        parent_id: u64,
    ) -> Self {
        ValueBuilder {
            state: ExclusiveState::new(writer, active_id, next_id, parent_id),
            wrote_something: false,
        }
    }

    /// Starts building a JSON object. Mirrors `ValueBuilder.object()`.
    pub fn object(mut self) -> io::Result<ObjectBuilder> {
        self.perform_single_write()?;
        let writer = self.state.writer.clone();
        let active_id = self.state.active_id.clone();
        let next_id = self.state.next_id.clone();
        let parent_id = self.state.id;
        self.state.closed.set(true);
        ObjectBuilder::new(writer, active_id, next_id, parent_id)
    }

    /// Starts building a JSON array. Mirrors `ValueBuilder.array()`.
    pub fn array(mut self) -> io::Result<ArrayBuilder> {
        self.perform_single_write()?;
        let writer = self.state.writer.clone();
        let active_id = self.state.active_id.clone();
        let next_id = self.state.next_id.clone();
        let parent_id = self.state.id;
        self.state.closed.set(true);
        ArrayBuilder::new(writer, active_id, next_id, parent_id)
    }

    /// Directly produces the given object as a JSON value. Mirrors
    /// `ValueBuilder.value(Object)`.
    pub fn value(mut self, value: &JsonValue) -> io::Result<()> {
        self.state.check_access()?;
        self.perform_single_write()?;
        self.state.writer_mut().print(value)?;
        self.finish_inner()
    }

    /// Register that this instance has started writing a value. Mirrors
    /// `ValueBuilder.performSingleWrite()`.
    fn perform_single_write(&mut self) -> io::Result<()> {
        if self.wrote_something {
            return Err(io::Error::other(
                "ValueBuilder instance attempted to write a second value",
            ));
        }
        self.wrote_something = true;
        Ok(())
    }

    fn finish_inner(&mut self) -> io::Result<()> {
        if !self.wrote_something {
            return Err(io::Error::other(
                "ValueBuilder instance was closed before writing anything",
            ));
        }
        self.state.finish()
    }
}

impl Drop for ValueBuilder {
    fn drop(&mut self) {
        if !self.state.closed.get() && self.wrote_something {
            let _ = self.state.finish();
        }
    }
}
