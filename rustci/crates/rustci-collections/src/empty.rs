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

    fn get_equivalence_strategy(&self) -> Equivalence {
        Equivalence::DEFAULT
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
