/*
 * Copyright (c) 2021, 2026, Oracle and/or its affiliates. All rights reserved.
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
// JSON value types shared between `JsonParser` and `JsonBuilder`.
//
// Java's `JsonParser` returns `Object` which can be `EconomicMap`, `List`,
// `String`, `Number`, `Boolean`, or `null`. Rust uses a discriminated union
// (`JsonValue`) to represent the same set of possible JSON values.

use rustci_collections::{BTreeEconomicMap, EconomicMap, UnmodifiableEconomicMap};
use std::fmt;

/// Represents a parsed JSON value. Mirrors Java's `Object` return type from
/// `JsonParser.parseLiteral()`.
pub enum JsonValue {
    /// JSON `null`. Mirrors Java `null`.
    Null,
    /// JSON boolean. Mirrors Java `Boolean`.
    Bool(bool),
    /// JSON string. Mirrors Java `String`.
    String(String),
    /// JSON number. Mirrors Java `Number` (Integer/Long/Double).
    Number(JsonNumber),
    /// JSON array. Mirrors Java `List<Object>`.
    Array(Vec<JsonValue>),
    /// JSON object. Mirrors Java `EconomicMap<String, Object>`.
    Object(BTreeEconomicMap<String, JsonValue>),
}

/// Represents a JSON number value. Mirrors Java's `Number` type with the
/// three concrete subtypes used by `JsonParser` (`Integer`, `Long`, `Double`).
#[derive(Clone, Debug, PartialEq)]
pub enum JsonNumber {
    /// Mirrors `Integer.parseInt` / `(int) longValue`.
    Int(i32),
    /// Mirrors `Long.parseLong` when the value exceeds `i32` range.
    Long(i64),
    /// Mirrors `Double.parseDouble` for floating-point values.
    Double(f64),
}

// ── Manual Clone for JsonValue ───────────────────────────────────────────

impl Clone for JsonValue {
    fn clone(&self) -> Self {
        match self {
            JsonValue::Null => JsonValue::Null,
            JsonValue::Bool(b) => JsonValue::Bool(*b),
            JsonValue::String(s) => JsonValue::String(s.clone()),
            JsonValue::Number(n) => JsonValue::Number(n.clone()),
            JsonValue::Array(arr) => JsonValue::Array(arr.clone()),
            JsonValue::Object(map) => {
                let mut new_map = BTreeEconomicMap::create();
                let mut cursor = map.get_entries();
                while cursor.advance() {
                    let key = cursor.get_key().clone();
                    let val = cursor.get_value().cloned();
                    new_map.put(key, val);
                }
                JsonValue::Object(new_map)
            }
        }
    }
}

// ── Manual Debug for JsonValue ───────────────────────────────────────────

impl fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "Null"),
            JsonValue::Bool(b) => f.debug_tuple("Bool").field(b).finish(),
            JsonValue::String(s) => f.debug_tuple("String").field(s).finish(),
            JsonValue::Number(n) => f.debug_tuple("Number").field(n).finish(),
            JsonValue::Array(arr) => f.debug_list().entries(arr.iter()).finish(),
            JsonValue::Object(map) => {
                let mut d = f.debug_map();
                let mut cursor = map.get_entries();
                while cursor.advance() {
                    let key = cursor.get_key();
                    let val = cursor.get_value();
                    let val_debug: &dyn fmt::Debug = match val {
                        Some(v) => v,
                        None => &"null",
                    };
                    d.entry(key, val_debug);
                }
                d.finish()
            }
        }
    }
}

// ── Manual PartialEq for JsonValue ───────────────────────────────────────

impl PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsonValue::Null, JsonValue::Null) => true,
            (JsonValue::Bool(a), JsonValue::Bool(b)) => a == b,
            (JsonValue::String(a), JsonValue::String(b)) => a == b,
            (JsonValue::Number(a), JsonValue::Number(b)) => a == b,
            (JsonValue::Array(a), JsonValue::Array(b)) => a == b,
            (JsonValue::Object(a), JsonValue::Object(b)) => {
                if a.size() != b.size() {
                    return false;
                }
                let mut cursor = a.get_entries();
                while cursor.advance() {
                    let key = cursor.get_key();
                    let val_a = cursor.get_value();
                    let val_b = b.get(key);
                    if val_a != val_b {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl fmt::Display for JsonNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonNumber::Int(i) => write!(f, "{}", i),
            JsonNumber::Long(l) => write!(f, "{}", l),
            JsonNumber::Double(d) => {
                if d.is_nan() || d.is_infinite() {
                    write!(f, "null")
                } else {
                    write!(f, "{}", d)
                }
            }
        }
    }
}
