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
// Port of `jdk.graal.compiler.util.json.test.JsonWriterTest`.

use crate::{JsonNumber, JsonValue, JsonWriterTrait};
use rustci_collections::{BTreeEconomicMap, EconomicMap, UnmodifiableEconomicMap};
use std::io;

// ── TestWriter ───────────────────────────────────────────────────────────
// A simple `JsonWriter` implementation backed by `Vec<u8>` for testing.

struct TestWriter {
    buf: Vec<u8>,
}

impl TestWriter {
    fn new() -> Self {
        TestWriter { buf: Vec::new() }
    }

    fn into_string(self) -> String {
        String::from_utf8(self.buf).unwrap()
    }

    fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }
}

impl JsonWriterTrait for TestWriter {
    fn append_object_start(&mut self) -> io::Result<()> {
        self.buf.push(b'{');
        Ok(())
    }

    fn append_object_end(&mut self) -> io::Result<()> {
        self.buf.push(b'}');
        Ok(())
    }

    fn append_array_start(&mut self) -> io::Result<()> {
        self.buf.push(b'[');
        Ok(())
    }

    fn append_array_end(&mut self) -> io::Result<()> {
        self.buf.push(b']');
        Ok(())
    }

    fn append_separator(&mut self) -> io::Result<()> {
        self.buf.push(b',');
        Ok(())
    }

    fn append_field_separator(&mut self) -> io::Result<()> {
        self.buf.push(b':');
        Ok(())
    }

    fn print(&mut self, value: &JsonValue) -> io::Result<()> {
        match value {
            JsonValue::Null => {
                self.write_bytes(b"null");
                Ok(())
            }
            JsonValue::Bool(true) => {
                self.write_bytes(b"true");
                Ok(())
            }
            JsonValue::Bool(false) => {
                self.write_bytes(b"false");
                Ok(())
            }
            JsonValue::String(s) => {
                self.buf.push(b'"');
                self.append_quoted_string(s)?;
                self.buf.push(b'"');
                Ok(())
            }
            JsonValue::Number(n) => match n {
                JsonNumber::Int(i) => {
                    self.write_bytes(i.to_string().as_bytes());
                    Ok(())
                }
                JsonNumber::Long(l) => {
                    self.write_bytes(l.to_string().as_bytes());
                    Ok(())
                }
                JsonNumber::Double(d) => {
                    if d.is_nan() || d.is_infinite() {
                        self.write_bytes(b"null");
                    } else {
                        self.write_bytes(d.to_string().as_bytes());
                    }
                    Ok(())
                }
            },
            JsonValue::Array(arr) => {
                self.append_array_start()?;
                let mut first = true;
                for v in arr {
                    if !first {
                        self.append_separator()?;
                    }
                    first = false;
                    self.print(v)?;
                }
                self.append_array_end()
            }
            JsonValue::Object(map) => {
                self.append_object_start()?;
                let mut cursor = map.get_entries();
                let mut first = true;
                while cursor.advance() {
                    if !first {
                        self.append_separator()?;
                    }
                    first = false;
                    self.append_raw(b"\"")?;
                    self.append_quoted_string(cursor.get_key())?;
                    self.append_raw(b"\"")?;
                    self.append_field_separator()?;
                    if let Some(v) = cursor.get_value() {
                        self.print(v)?;
                    } else {
                        self.write_bytes(b"null");
                    }
                }
                self.append_object_end()
            }
        }
    }

    fn append_raw(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_bytes(data);
        Ok(())
    }

    fn append_quoted_string(&mut self, s: &str) -> io::Result<()> {
        for c in s.chars() {
            match c {
                '"' => self.write_bytes(b"\\\""),
                '\\' => self.write_bytes(b"\\\\"),
                '\x08' => self.write_bytes(b"\\b"),
                '\x0c' => self.write_bytes(b"\\f"),
                '\n' => self.write_bytes(b"\\n"),
                '\r' => self.write_bytes(b"\\r"),
                '\t' => self.write_bytes(b"\\t"),
                c if c < ' ' => {
                    let escaped = format!("\\u{:04x}", c as u32);
                    self.write_bytes(escaped.as_bytes());
                }
                c => {
                    let mut buf = [0u8; 4];
                    let encoded = c.encode_utf8(&mut buf);
                    self.write_bytes(encoded.as_bytes());
                }
            }
        }
        Ok(())
    }
}

/// Helper: serialize a JsonValue to a compact JSON string.
fn to_json(value: &JsonValue) -> String {
    let mut writer = TestWriter::new();
    writer.print(value).unwrap();
    writer.into_string()
}

/// Helper to build a JsonValue int.
fn jv_int(i: i32) -> JsonValue {
    JsonValue::Number(JsonNumber::Int(i))
}

/// Helper to build a JsonValue double.
fn jv_double(d: f64) -> JsonValue {
    JsonValue::Number(JsonNumber::Double(d))
}

/// Helper to build a JsonValue string.
fn jv_str(s: &str) -> JsonValue {
    JsonValue::String(s.to_string())
}

/// Helper to build a JsonValue array.
fn jv_array(items: Vec<JsonValue>) -> JsonValue {
    JsonValue::Array(items)
}

// ── testNull ─────────────────────────────────────────────────────────────
#[test]
fn test_null() {
    // Java: Assert.assertEquals("null", toJson(null));
    assert_eq!("null", to_json(&JsonValue::Null));
    // Java: Assert.assertEquals("\"null\"", toJson("null"));
    assert_eq!("\"null\"", to_json(&jv_str("null")));
}

// ── testString ───────────────────────────────────────────────────────────
#[test]
fn test_string() {
    // Java: Assert.assertEquals("\"\"", toJson(""));
    assert_eq!("\"\"", to_json(&jv_str("")));
    // Java: Assert.assertEquals("\"string\"", toJson("string"));
    assert_eq!("\"string\"", to_json(&jv_str("string")));
    // Java: Assert.assertEquals("\"42\"", toJson("42"));
    assert_eq!("\"42\"", to_json(&jv_str("42")));
}

// ── testQuoting ──────────────────────────────────────────────────────────
#[test]
fn test_quoting() {
    // Java: Assert.assertEquals("\"\\\"\"", toJson("\""));
    assert_eq!("\"\\\"\"", to_json(&jv_str("\"")));
    // Java: Assert.assertEquals("\"\\t\\n\"", toJson("\t\n"));
    assert_eq!("\"\\t\\n\"", to_json(&jv_str("\t\n")));
    // Java: Assert.assertEquals("\"'\"", toJson("'"));
    assert_eq!("\"'\"", to_json(&jv_str("'")));
}

// ── testNumber ───────────────────────────────────────────────────────────
#[test]
fn test_number() {
    // Java: Assert.assertEquals("0", toJson(0));
    assert_eq!("0", to_json(&jv_int(0)));
    // Java: Assert.assertEquals("42", toJson(42));
    assert_eq!("42", to_json(&jv_int(42)));
    // Java: Assert.assertEquals("42.5", toJson(42.5f));
    assert_eq!("42.5", to_json(&jv_double(42.5)));
    // Java: Assert.assertEquals("42.5", toJson(42.5));
    assert_eq!("42.5", to_json(&jv_double(42.5)));
}

// ── testBoolean ──────────────────────────────────────────────────────────
#[test]
fn test_boolean() {
    // Java: Assert.assertEquals("true", toJson(true));
    assert_eq!("true", to_json(&JsonValue::Bool(true)));
    // Java: Assert.assertEquals("false", toJson(false));
    assert_eq!("false", to_json(&JsonValue::Bool(false)));
    // Java: Assert.assertEquals("\"false\"", toJson("false"));
    assert_eq!("\"false\"", to_json(&jv_str("false")));
}

// ── testList0 ────────────────────────────────────────────────────────────
#[test]
fn test_list0() {
    // Java: var input = List.of();  Assert.assertEquals("[]", toJson(input));
    assert_eq!("[]", to_json(&JsonValue::Array(vec![])));
}

// ── testList1 ────────────────────────────────────────────────────────────
#[test]
fn test_list1() {
    // Java: var input = List.of(1, 2, 3);  Assert.assertEquals("[1,2,3]", toJson(input));
    assert_eq!(
        "[1,2,3]",
        to_json(&jv_array(vec![jv_int(1), jv_int(2), jv_int(3)]))
    );
}

// ── testList2 ────────────────────────────────────────────────────────────
#[test]
fn test_list2() {
    // Java: var input = List.of(1, List.of(2, 3));  Assert.assertEquals("[1,[2,3]]", toJson(input));
    assert_eq!(
        "[1,[2,3]]",
        to_json(&jv_array(vec![
            jv_int(1),
            jv_array(vec![jv_int(2), jv_int(3)]),
        ]))
    );
}

// ── testMap0 ─────────────────────────────────────────────────────────────
#[test]
fn test_map0() {
    // Java: var input = new EconomicHashMap<>();  Assert.assertEquals("{}", toJson(input));
    let map: BTreeEconomicMap<String, JsonValue> = BTreeEconomicMap::create();
    assert_eq!("{}", to_json(&JsonValue::Object(map)));
}

// ── testMap1 ─────────────────────────────────────────────────────────────
#[test]
fn test_map1() {
    // Java: var input = CollectionsUtil.mapOf("k1", 1, "k2", 2);
    //        var output = toJson(input);
    //        Assert.assertTrue("{\"k1\":1,\"k2\":2}".equals(output) || "{\"k2\":2,\"k1\":1}".equals(output));
    let mut map: BTreeEconomicMap<String, JsonValue> = BTreeEconomicMap::create();
    map.put("k1".to_string(), Some(jv_int(1)));
    map.put("k2".to_string(), Some(jv_int(2)));
    let output = to_json(&JsonValue::Object(map));
    assert!(
        output == "{\"k1\":1,\"k2\":2}" || output == "{\"k2\":2,\"k1\":1}",
        "Unexpected output: {}",
        output
    );
}

// ── testMap2 ─────────────────────────────────────────────────────────────
#[test]
fn test_map2() {
    // Java: var input = CollectionsUtil.mapOf("k1", List.of(1, 2, 3), "k2", List.of());
    //        var output = toJson(input);
    //        Assert.assertTrue("{\"k1\":[1,2,3],\"k2\":[]}".equals(output) || "{\"k2\":[],\"k1\":[1,2,3]}".equals(output));
    let mut map: BTreeEconomicMap<String, JsonValue> = BTreeEconomicMap::create();
    map.put(
        "k1".to_string(),
        Some(jv_array(vec![jv_int(1), jv_int(2), jv_int(3)])),
    );
    map.put("k2".to_string(), Some(jv_array(vec![])));
    let output = to_json(&JsonValue::Object(map));
    assert!(
        output == "{\"k1\":[1,2,3],\"k2\":[]}" || output == "{\"k2\":[],\"k1\":[1,2,3]}",
        "Unexpected output: {}",
        output
    );
}

// ── testMap3 ─────────────────────────────────────────────────────────────
#[test]
fn test_map3() {
    // Java: var input = CollectionsUtil.mapOf("k1", CollectionsUtil.mapOf("k1", List.of(1, 2, 3)), "k2", "");
    //        var output = toJson(input);
    //        Assert.assertTrue("{\"k1\":{\"k1\":[1,2,3]},\"k2\":\"\"}".equals(output)
    //                       || "{\"k2\":\"\",\"k1\":{\"k1\":[1,2,3]}}".equals(output));
    let mut inner: BTreeEconomicMap<String, JsonValue> = BTreeEconomicMap::create();
    inner.put(
        "k1".to_string(),
        Some(jv_array(vec![jv_int(1), jv_int(2), jv_int(3)])),
    );
    let mut map: BTreeEconomicMap<String, JsonValue> = BTreeEconomicMap::create();
    map.put("k1".to_string(), Some(JsonValue::Object(inner)));
    map.put("k2".to_string(), Some(jv_str("")));
    let output = to_json(&JsonValue::Object(map));
    assert!(
        output == "{\"k1\":{\"k1\":[1,2,3]},\"k2\":\"\"}"
            || output == "{\"k2\":\"\",\"k1\":{\"k1\":[1,2,3]}}",
        "Unexpected output: {}",
        output
    );
}
