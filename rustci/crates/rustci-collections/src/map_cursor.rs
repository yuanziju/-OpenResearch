/*
 * Copyright (c) 2017, 2022, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.UnmodifiableMapCursor<K, V>` and
// `org.graalvm.collections.MapCursor<K, V>`.
//
// `MapCursor extends UnmodifiableMapCursor`. The base trait exposes
// `advance` / `getKey` / `getValue`; the mutable subtrait adds `remove`
// (abstract) and `setValue` (default `UnsupportedOperationException`).
//
// Null-value semantics: Java's `getValue()` may return `null` for an entry
// whose value is null. In Rust the value type `V` is the non-null carrier and
// null is represented by `Option<V>` throughout the collections crate, so
// `get_value` returns `Option<&V>` (`None` for a null-value entry). Mirrors
// Java returning `null` from `getValue()` for such entries.

/// Cursor to iterate over a map without changing its contents. Mirrors
/// `org.graalvm.collections.UnmodifiableMapCursor<K, V>`.
pub trait UnmodifiableMapCursor<K, V> {
    /// Advances to the next entry. Returns `true` if a next entry exists,
    /// `false` if there is no next entry. Mirrors `advance()`.
    fn advance(&mut self) -> bool;

    /// The key of the current entry. Panics (mirroring Java
    /// `NoSuchElementException`) if no entry is current. Mirrors `getKey()`.
    fn get_key(&self) -> &K;

    /// The value of the current entry. Returns `None` if the current entry's
    /// value is null. Panics (mirroring Java `NoSuchElementException`) if no
    /// entry is current. Mirrors `getValue()`.
    fn get_value(&self) -> Option<&V>;
}

/// Cursor to iterate over a mutable map. Mirrors
/// `org.graalvm.collections.MapCursor<K, V>`.
pub trait MapCursor<K, V>: UnmodifiableMapCursor<K, V> {
    /// Remove the current entry from the map. May only be called once. After
    /// calling `remove`, it is no longer valid to call `get_key` or
    /// `get_value` on the current entry. Mirrors `remove()`.
    fn remove(&mut self);

    /// Set the value of the current entry. Returns the previous value
    /// associated with the current key (`None` if it was null). Default
    /// implementation throws `UnsupportedOperationException`, matching Java.
    fn set_value(&mut self, _new_value: Option<V>) -> Option<V> {
        panic!("UnsupportedOperationException: setValue not supported");
    }
}
