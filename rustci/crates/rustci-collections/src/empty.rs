/*
 * Copyright (c) 2022, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.EmptyMap` and
// `org.graalvm.collections.EmptySet`.
//
// Java provides package-private singleton instances `EmptyMap.EMPTY_MAP`,
// `EmptyMap.EMPTY_CURSOR`, `EmptyMap.EMPTY_ITERATOR`, `EmptySet.EMPTY_SET`.
// They are always-empty, always-unmodifiable views whose mutating operations
// throw `IllegalArgumentException("Cannot modify the always-empty map/set")`
// and whose cursor accessors throw
// `NoSuchElementException("Empty cursor does not have elements")`. Here they
// are mirrored as zero-sized generic structs returned by the
// `EconomicMap::empty_map` / `EconomicMap::empty_cursor` /
// `EconomicSet::empty_set` factories.

use crate::economic_map::{check_non_null_key, EconomicMap, UnmodifiableEconomicMap};
use crate::economic_set::{check_non_null_element, EconomicSet, UnmodifiableEconomicSet};
use crate::equivalence::Equivalence;
use crate::map_cursor::{MapCursor, UnmodifiableMapCursor};
use core::marker::PhantomData;

/// Always-empty cursor. Mirrors `EmptyMap.EMPTY_CURSOR`. All accessors panic
/// with `NoSuchElementException("Empty cursor does not have elements")`,
/// matching Java; `advance` always returns `false`.
#[derive(Debug)]
pub struct EmptyCursor<K, V> {
    _phantom: PhantomData<(K, V)>,
}

// Manual `Default` impl without bounds on `K` / `V` (PhantomData is always
// Default), so `BTreeEconomicMap::empty_cursor()` does not require
// `K: Default + V: Default`.
impl<K, V> Default for EmptyCursor<K, V> {
    fn default() -> Self {
        EmptyCursor {
            _phantom: PhantomData,
        }
    }
}

impl<K, V> UnmodifiableMapCursor<K, V> for EmptyCursor<K, V> {
    fn advance(&mut self) -> bool {
        false
    }

    fn get_key(&self) -> &K {
        panic!("NoSuchElementException: Empty cursor does not have elements");
    }

    fn get_value(&self) -> Option<&V> {
        panic!("NoSuchElementException: Empty cursor does not have elements");
    }
}

impl<K, V> MapCursor<K, V> for EmptyCursor<K, V> {
    fn remove(&mut self) {
        panic!("NoSuchElementException: Empty cursor does not have elements");
    }

    // Override the trait default (which throws `UnsupportedOperationException`)
    // to align with Java's `EmptyMap.EMPTY_CURSOR.setValue`, which throws
    // `NoSuchElementException("Empty cursor does not have elements")` — the
    // same exception as `getKey` / `getValue` / `remove`.
    fn set_value(&mut self, _new_value: Option<V>) -> Option<V> {
        panic!("NoSuchElementException: Empty cursor does not have elements");
    }
}

/// Always-empty, always-unmodifiable `EconomicMap`. Mirrors
/// `EmptyMap.EMPTY_MAP`. Mutating operations throw
/// `IllegalArgumentException("Cannot modify the always-empty map")`; lookups
/// return empty/`None`.
#[derive(Debug)]
pub struct EmptyMap<K, V> {
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> Default for EmptyMap<K, V> {
    fn default() -> Self {
        EmptyMap {
            _phantom: PhantomData,
        }
    }
}

impl<K, V> UnmodifiableEconomicMap<K, V> for EmptyMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        check_non_null_key(key);
        None
    }

    fn get_or_default<'a>(&'a self, key: &K, default_value: &'a V) -> Option<&'a V> {
        check_non_null_key(key);
        Some(default_value)
    }

    fn contains_key(&self, key: &K) -> bool {
        check_non_null_key(key);
        false
    }

    fn size(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn get_values(&self) -> Box<dyn Iterator<Item = Option<&V>> + '_> {
        Box::new(core::iter::empty())
    }

    fn get_keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(core::iter::empty())
    }

    fn get_entries(&self) -> Box<dyn UnmodifiableMapCursor<K, V> + '_> {
        Box::new(EmptyCursor::<K, V>::default())
    }

    fn get_equivalence_strategy(&self) -> Equivalence {
        Equivalence::DEFAULT
    }
}

impl<K, V> EconomicMap<K, V> for EmptyMap<K, V> {
    fn put(&mut self, key: K, _value: Option<V>) -> Option<V> {
        check_non_null_key(&key);
        panic!("IllegalArgumentException: Cannot modify the always-empty map");
    }

    fn clear(&mut self) {
        panic!("IllegalArgumentException: Cannot modify the always-empty map");
    }

    fn remove_key(&mut self, key: &K) -> Option<V> {
        check_non_null_key(key);
        panic!("IllegalArgumentException: Cannot modify the always-empty map");
    }

    fn replace_all(&mut self, _function: &dyn Fn(&K, Option<&V>) -> Option<V>) {
        panic!("IllegalArgumentException: Cannot modify the always-empty map");
    }
}

/// Always-empty, always-unmodifiable `EconomicSet`. Mirrors
/// `EmptySet.EMPTY_SET`. Mutating operations throw
/// `IllegalArgumentException("Cannot modify the always-empty set")`; lookups
/// return empty/`false`.
#[derive(Debug)]
pub struct EmptySet<E> {
    _phantom: PhantomData<E>,
}

impl<E> Default for EmptySet<E> {
    fn default() -> Self {
        EmptySet {
            _phantom: PhantomData,
        }
    }
}

impl<E> UnmodifiableEconomicSet<E> for EmptySet<E> {
    fn contains(&self, element: &E) -> bool {
        check_non_null_element(element);
        false
    }

    fn size(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &E> + '_> {
        Box::new(core::iter::empty())
    }
}

impl<E> EconomicSet<E> for EmptySet<E> {
    fn add(&mut self, element: E) -> bool {
        check_non_null_element(&element);
        panic!("IllegalArgumentException: Cannot modify the always-empty set");
    }

    fn remove(&mut self, element: &E) {
        check_non_null_element(element);
        panic!("IllegalArgumentException: Cannot modify the always-empty set");
    }

    fn clear(&mut self) {
        panic!("IllegalArgumentException: Cannot modify the always-empty set");
    }
}
