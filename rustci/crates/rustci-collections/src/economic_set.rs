/*
 * Copyright (c) 2017, 2026, Oracle and/or its affiliates. All rights reserved.
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
// Port of `org.graalvm.collections.UnmodifiableEconomicSet<E>` and
// `org.graalvm.collections.EconomicSet<E>`.
//
// Java interface → Rust trait; Java `default` method → trait default impl.
// `UnmodifiableEconomicSet extends Iterable<E>`, so the base trait exposes an
// `iterator` method (Rust analog: `Box<dyn Iterator<Item = &E> + '_>`). The set
// does not support a `null` element (mirrors Java: looking up / adding a null
// element throws `UnsupportedOperationException`).
//
// Null-element handling mirrors the map's null-key handling: Rust keys/elements
// cannot be language-level null, so the `UnsupportedOperationException` guard
// is preserved as a behavioral anchor (see `check_non_null_element`).

/// Reject a `null` element by panicking, mirroring
/// `EconomicMapImpl.checkNonNull` → `UnsupportedOperationException("null not
/// supported")`. Retained as a behavioral anchor; Rust element types cannot be
/// language-level null.
#[track_caller]
pub(crate) fn check_non_null_element<T>(element: &T) {
    let _ = element;
}

/// Unmodifiable memory efficient set data structure. Mirrors
/// `org.graalvm.collections.UnmodifiableEconomicSet<E>`, extending `Iterable<E>`.
pub trait UnmodifiableEconomicSet<E> {
    /// Returns `true` if this set contains a mapping for the `element`. The
    /// `element` must not be `null`. Mirrors `boolean contains(E element)`.
    fn contains(&self, element: &E) -> bool;

    /// Returns the number of elements in this set. Mirrors `int size()`.
    fn size(&self) -> usize;

    /// Returns `true` if this set contains no elements. Mirrors
    /// `boolean isEmpty()`.
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns an iterator over the elements of this set. Mirrors
    /// `Iterable<E>.iterator()`.
    fn iterator(&self) -> Box<dyn Iterator<Item = &E> + '_>;

    /// Stores all the elements in this set into `target`. Panics (mirroring
    /// Java `UnsupportedOperationException`) if the length of `target` does
    /// not equal the size of this set. Mirrors `E[] toArray(E[] target)`.
    fn to_array(&self, target: &mut [E])
    where
        E: Clone,
    {
        if target.len() != self.size() {
            panic!("UnsupportedOperationException: Length of target array must equal the size of the set.");
        }
        for (index, element) in self.iterator().enumerate() {
            target[index] = element.clone();
        }
    }

    /// Returns a `HashSet` containing all elements of this set. Mirrors
    /// `HashSet<E> toHashSet()`.
    fn to_hash_set(&self) -> std::collections::HashSet<E>
    where
        E: Clone + std::hash::Hash + Eq,
    {
        let mut set = std::collections::HashSet::with_capacity(self.size());
        for elem in self.iterator() {
            set.insert(elem.clone());
        }
        set
    }

    /// Returns a `Vec` containing all elements of this set in iteration
    /// order. Mirrors `List<E> toList()`.
    fn to_list(&self) -> Vec<E>
    where
        E: Clone,
    {
        let mut list = Vec::with_capacity(self.size());
        for elem in self.iterator() {
            list.push(elem.clone());
        }
        list
    }

    /// Returns `true` if this set contains all of the elements in `coll`.
    /// Mirrors `boolean containsAll(Iterable<? extends E> coll)`. The Java
    /// overload takes `Iterable<? extends E>`; in Rust a slice is the closest
    /// ergonomic analog (deviation recorded).
    fn contains_all(&self, coll: &[E]) -> bool
    where
        E: PartialEq,
    {
        for e in coll {
            if !self.contains(e) {
                return false;
            }
        }
        true
    }

    /// Removes all elements of this set that satisfy the given predicate.
    /// Returns `true` if any elements were removed. Mirrors
    /// `boolean removeIf(Predicate<? super E> filter)`. In Java this is
    /// inherited from `Iterable` and uses `Iterator.remove()`; in Rust,
    /// `std::iter::Iterator` has no `remove`, so the default collects elements
    /// to remove and calls `Self::remove`. The `Self: EconomicSet<E>` bound
    /// restricts this to mutable sets (matching Java's runtime behavior where
    /// an unmodifiable set's iterator throws `UnsupportedOperationException`).
    fn remove_if(&mut self, filter: &dyn Fn(&E) -> bool) -> bool
    where
        E: Clone,
        Self: Sized + EconomicSet<E>,
    {
        let mut to_remove: Vec<E> = Vec::new();
        for elem in self.iterator() {
            if filter(elem) {
                to_remove.push(elem.clone());
            }
        }
        let removed = !to_remove.is_empty();
        for elem in to_remove {
            self.remove(&elem);
        }
        removed
    }
}

/// Memory efficient set data structure. Mirrors
/// `org.graalvm.collections.EconomicSet<E>`, extending
/// `UnmodifiableEconomicSet<E>`. It does not support adding, looking up or
/// removing a `null` element.
pub trait EconomicSet<E>: UnmodifiableEconomicSet<E> {
    /// Adds non-null `element` to this set if it is not already present.
    /// Returns `true` if this set did not already contain `element`. Mirrors
    /// `boolean add(E element)`.
    fn add(&mut self, element: E) -> bool;

    /// Removes non-null `element` from this set if it is present. This set
    /// will not contain `element` once the call returns. Mirrors
    /// `void remove(E element)`.
    fn remove(&mut self, element: &E);

    /// Removes all the elements from this set. The set will be empty after
    /// this call returns. Mirrors `void clear()`.
    fn clear(&mut self);

    /// Adds all the elements in `other` to this set if they're not already
    /// present. Mirrors `void addAll(EconomicSet<? extends E> other)`.
    fn add_all_set(&mut self, other: &dyn EconomicSet<E>)
    where
        E: Clone,
        Self: Sized,
    {
        for elem in other.iterator() {
            self.add(elem.clone());
        }
    }

    /// Adds all the elements in `values` to this set if they're not already
    /// present. Mirrors `void addAll(Iterable<? extends E> values)` and
    /// `void addAll(Iterator<? extends E> iterator)` (Java overloads
    /// consolidated into the slice variant; deviation recorded).
    fn add_all_slice(&mut self, values: &[E])
    where
        E: Clone,
    {
        for v in values {
            self.add(v.clone());
        }
    }

    /// Removes from this set all of its elements that are contained in
    /// `other`. Mirrors `void removeAll(EconomicSet<E> other)`.
    fn remove_all_set(&mut self, other: &dyn EconomicSet<E>)
    where
        E: Clone,
        Self: Sized,
    {
        let to_remove: Vec<E> = other.iterator().cloned().collect();
        for elem in to_remove {
            self.remove(&elem);
        }
    }

    /// Removes from this set all of its elements that are contained in
    /// `values`. Mirrors `void removeAll(Iterable<E> values)` and
    /// `void removeAll(Iterator<E> iterator)` (Java overloads consolidated
    /// into the slice variant; deviation recorded).
    fn remove_all_slice(&mut self, values: &[E])
    where
        E: Clone,
    {
        for v in values {
            self.remove(v);
        }
    }

    /// Removes from this set all of its elements that are not contained in
    /// `other`. Mirrors `void retainAll(EconomicSet<E> other)`.
    fn retain_all(&mut self, other: &dyn EconomicSet<E>)
    where
        E: Clone,
        Self: Sized,
    {
        let to_remove: Vec<E> = self
            .iterator()
            .filter(|e| !other.contains(e))
            .cloned()
            .collect();
        for elem in to_remove {
            self.remove(&elem);
        }
    }
}
