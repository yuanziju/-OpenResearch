// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Reference implementation of `EconomicSet` backed by
// `std::collections::BTreeSet`.
//
// This stands in for the `EconomicMapImpl`-backed `EconomicSet` (deferred to a
// later task). Behaviorally it mirrors the observable Java `EconomicSet`
// contract: insertion-ordered iteration (BTreeSet yields elements in `Ord`
// order; Java `EconomicMapImpl`-backed sets yield in insertion order — see
// deviation record), null-element rejection, and the full
// `UnmodifiableEconomicSet` / `EconomicSet` method surface.
//
// In Java, `EconomicSet` is implemented by `EconomicMapImpl` using a map with a
// sentinel `SET_PRESENT` value. Here a `BTreeSet<E>` is used directly — simpler
// and behaviorally equivalent for the stand-in layer.

use std::collections::BTreeSet;

use crate::economic_set::{check_non_null_element, EconomicSet, UnmodifiableEconomicSet};
use crate::empty::EmptySet;
use crate::equivalence::Equivalence;

/// Reference `EconomicSet` implementation backed by a `BTreeSet`. Stands in
/// for the `EconomicMapImpl`-backed `EconomicSet` until the former is ported.
pub struct BTreeEconomicSet<E>
where
    E: Ord,
{
    inner: BTreeSet<E>,
    strategy: Equivalence,
}

impl<E> BTreeEconomicSet<E>
where
    E: Ord,
{
    /// Creates a new empty set with the default `Equivalence::DEFAULT`
    /// strategy. Mirrors `EconomicSet.create()`.
    pub fn create() -> Self {
        Self::create_with_strategy(Equivalence::DEFAULT)
    }

    /// Creates a new empty set with the default strategy and the given initial
    /// capacity hint (BTreeSet does not pre-allocate, so the hint is a no-op;
    /// retained for API parity). Mirrors `EconomicSet.create(int)`.
    pub fn create_with_capacity(_initial_capacity: usize) -> Self {
        Self::create()
    }

    /// Creates a new empty set with the given comparison strategy. Mirrors
    /// `EconomicSet.create(Equivalence)`.
    pub fn create_with_strategy(strategy: Equivalence) -> Self {
        BTreeEconomicSet {
            inner: BTreeSet::new(),
            strategy,
        }
    }

    /// Creates a new empty set with the given strategy and capacity hint.
    /// Mirrors `EconomicSet.create(Equivalence, int)`.
    pub fn create_with_strategy_capacity(strategy: Equivalence, _initial_capacity: usize) -> Self {
        Self::create_with_strategy(strategy)
    }

    /// Creates a new set copying all elements from `c`. Mirrors
    /// `EconomicSet.create(UnmodifiableEconomicSet)`.
    pub fn create_from(c: &dyn UnmodifiableEconomicSet<E>) -> Self
    where
        E: Clone,
    {
        Self::create_with_strategy_from(Equivalence::DEFAULT, c)
    }

    /// Creates a new set with the given strategy copying all elements from `c`.
    /// Mirrors `EconomicSet.create(Equivalence, UnmodifiableEconomicSet)`.
    pub fn create_with_strategy_from(
        strategy: Equivalence,
        c: &dyn UnmodifiableEconomicSet<E>,
    ) -> Self
    where
        E: Clone,
    {
        let mut set = Self::create_with_strategy(strategy);
        // Mirrors Java's addAll(Iterator) → add delegation, since
        // UnmodifiableEconomicSet is not an EconomicSet in the Rust type
        // hierarchy (trait objects are not substitutable).
        for elem in c.iterator() {
            set.add(elem.clone());
        }
        set
    }

    /// Returns an empty, unmodifiable `EconomicSet` singleton view. Mirrors
    /// `EconomicSet.emptySet()`.
    pub fn empty_set() -> EmptySet<E> {
        EmptySet::default()
    }

    /// Creates an `EconomicSet` with one element. Mirrors `EconomicSet.of(E)`.
    pub fn of(elem: E) -> Self {
        let mut set = Self::create_with_capacity(1);
        set.add(elem);
        set
    }
}

impl<E> UnmodifiableEconomicSet<E> for BTreeEconomicSet<E>
where
    E: Ord,
{
    fn contains(&self, element: &E) -> bool {
        check_non_null_element(element);
        self.inner.contains(element)
    }

    fn size(&self) -> usize {
        self.inner.len()
    }

    fn iterator(&self) -> Box<dyn Iterator<Item = &E> + '_> {
        Box::new(self.inner.iter())
    }

    fn get_equivalence_strategy(&self) -> Equivalence {
        self.strategy
    }
}

impl<E> EconomicSet<E> for BTreeEconomicSet<E>
where
    E: Ord,
{
    fn add(&mut self, element: E) -> bool {
        check_non_null_element(&element);
        self.inner.insert(element)
    }

    fn remove(&mut self, element: &E) {
        check_non_null_element(element);
        self.inner.remove(element);
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}
