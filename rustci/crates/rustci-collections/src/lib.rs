// SPDX-License-Identifier: UPL-1.0 OR GPL-2.0-with-classpath-exception
//
// Rust mirror of `org.graalvm.collections` (sdk module). Faithful 1:1 port of the
// interface surface (UnmodifiableEconomicMap / EconomicMap / MapCursor /
// UnmodifiableMapCursor / Equivalence / Pair / EconomicSet / UnmodifiableEconomicSet)
// plus a BTreeMap-based reference implementation (`BTreeEconomicMap` /
// `BTreeEconomicSet`). The full segmented-array `EconomicMapImpl` (941 LoC) is
// deferred to a later task; this crate stands in until then.

pub mod btree_economic_map;
pub mod btree_economic_set;
pub mod economic_map;
pub mod economic_map_wrap;
pub mod economic_set;
pub mod empty;
pub mod equivalence;
pub mod map_cursor;
pub mod pair;

pub use btree_economic_map::{BTreeEconomicMap, BTreeEconomicMapCursor};
pub use btree_economic_set::BTreeEconomicSet;
pub use economic_map::{EconomicMap, UnmodifiableEconomicMap};
pub use economic_map_wrap::EconomicMapWrap;
pub use economic_set::{EconomicSet, UnmodifiableEconomicSet};
pub use empty::{EmptyCursor, EmptyMap, EmptySet};
pub use equivalence::Equivalence;
pub use map_cursor::{MapCursor, UnmodifiableMapCursor};
pub use pair::Pair;

#[cfg(test)]
mod tests;
