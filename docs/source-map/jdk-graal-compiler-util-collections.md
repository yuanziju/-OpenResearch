# jdk.graal.compiler.util.collections

## 源位置：`/opt/graal/sdk/src/org.graalvm.collections/src/org/graalvm/collections/`（**sdk 模块**，包名 `org.graalvm.collections`；非 compiler 子目录）

## 状态：已完成

## 说明

`EconomicMap` / `EconomicSet` 等 nil-dependent 集合类型，是 graph / nodes / util.json 的基础数据结构。源在 **sdk 模块**（`org.graalvm.collections` 包），不在 compiler。本期（T2）完成全部接口 trait 镜像 + BTreeMap/BTreeSet 参考实现 + 单元测试。完整分段数组 `EconomicMapImpl`（941 LoC）移植列为后续任务。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
| `org.graalvm.collections.UnmodifiableEconomicMap<K,V>` | 只读 map 接口（get/containsKey/size/getEntries/getEquivalenceStrategy 等 + default getOrDefault/isEmpty） | `crates/rustci-collections/src/economic_map.rs` | 已完成 | T2 |
| `org.graalvm.collections.EconomicMap<K,V>` | 可变 map 接口（put/removeKey/clear/replaceAll + default putIfAbsent/putAll/computeIfAbsent/trimToSize + 静态工厂 create/of/emptyMap/wrapMap/emptyCursor） | `crates/rustci-collections/src/economic_map.rs`（trait）+ `crates/rustci-collections/src/btree_economic_map.rs`（BTreeMap 参考实现 + 工厂） | 已完成 | T2 |
| `org.graalvm.collections.UnmodifiableMapCursor<K,V>` | 只读游标接口（advance/getKey/getValue） | `crates/rustci-collections/src/map_cursor.rs` | 已完成 | T2 |
| `org.graalvm.collections.MapCursor<K,V>` | 可变游标接口（extends UnmodifiableMapCursor + remove/setValue default UnsupportedOperationException） | `crates/rustci-collections/src/map_cursor.rs` | 已完成 | T2 |
| `org.graalvm.collections.Equivalence` | 键比较/哈希策略枚举（DEFAULT/IDENTITY/IDENTITY_WITH_SYSTEM_HASHCODE + equals/hashCode） | `crates/rustci-collections/src/equivalence.rs` | 已完成 | T2 |
| `org.graalvm.collections.Pair<L,R>` | 不可变值对（empty/createLeft/createRight/create + getLeft/getRight + hashCode/equals/toString） | `crates/rustci-collections/src/pair.rs` | 已完成 | T2 |
| `org.graalvm.collections.UnmodifiableEconomicSet<E>` | 只读 set 接口（contains/size/iterator + default toArray/toHashSet/toList/containsAll/removeIf） | `crates/rustci-collections/src/economic_set.rs` | 已完成 | T2 |
| `org.graalvm.collections.EconomicSet<E>` | 可变 set 接口（add/remove/clear + default addAll/removeAll/retainAll + 静态工厂 create/of/emptySet） | `crates/rustci-collections/src/economic_set.rs`（trait）+ `crates/rustci-collections/src/btree_economic_set.rs`（BTreeSet 参考实现 + 工厂） | 已完成 | T2 |
| `org.graalvm.collections.EconomicMapWrap<K,V>` | 包装 `java.util.Map` 为 `EconomicMap` | `crates/rustci-collections/src/economic_map_wrap.rs` | 已完成 | T2 |
| `org.graalvm.collections.EmptyMap` | 空 map/cursor/iterator 单例（包私有） | `crates/rustci-collections/src/empty.rs`（EmptyMap/EmptyCursor/EmptySet） | 已完成 | T2 |
| `org.graalvm.collections.EmptySet` | 空 set 单例（包私有） | `crates/rustci-collections/src/empty.rs` | 已完成 | T2 |
| `org.graalvm.collections.EconomicMapImpl<K,V>` | 分段数组+哈希压缩实现（941 LoC） | **后续任务**（BTreeMap 暂替） | 待移植 | — |
| `org.graalvm.collections.EconomicMapUtil` | map 工具类（equals/hashCode/keySet/lexicographicalComparator） | **后续任务** | 待移植 | — |

## Rust crate 结构

```
crates/rustci-collections/src/
├── lib.rs                    # 模块声明 + 公开 API re-export
├── economic_map.rs           # UnmodifiableEconomicMap + EconomicMap traits + check_non_null_key
├── economic_set.rs           # UnmodifiableEconomicSet + EconomicSet traits + check_non_null_element
├── map_cursor.rs             # UnmodifiableMapCursor + MapCursor traits
├── equivalence.rs            # Equivalence enum (Default/Identity/IdentityWithSystemHashCode)
├── pair.rs                   # Pair<L,R> struct + impl
├── empty.rs                  # EmptyMap / EmptyCursor / EmptySet 单例
├── btree_economic_map.rs     # BTreeEconomicMap<K,V> 参考实现 + BTreeEconomicMapCursor + 工厂
├── btree_economic_set.rs     # BTreeEconomicSet<E> 参考实现 + 工厂
├── economic_map_wrap.rs      # EconomicMapWrap<K,V>（包装 BTreeMap 为 EconomicMap）
└── tests.rs                  # 79 个单元测试
```

Rust LoC：总计 2393 行（非测试 1486 行 + 测试 907 行）。

## 偏离记录

### 1. BTreeMap/BTreeSet 迭代顺序（已知偏离，待 EconomicMapImpl 移植修正）

- **Java**：`EconomicMapImpl` 保证 **插入顺序** 迭代（`getEntries`/`getKeys`/`getValues`/`iterator`）。
- **Rust**：`BTreeMap`/`BTreeSet` 按 **`Ord` 排序顺序** 迭代。
- **影响**：cursor/key/value/set 迭代顺序与 Java 不一致。
- **缓解**：测试中明确标注此偏离（`cursor_iterates_in_sorted_order`、`set_iterator_sorted_order`）。
- **修正**：完整 `EconomicMapImpl` 移植后将恢复插入顺序语义。

### 2. Cursor 变更操作(remove/setValue)分两类处理

- **Java**：`MapCursor.remove()` 和 `MapCursor.setValue()` 在 `EconomicMapImpl` 的 cursor 中直接变更底层存储；`EconomicMapWrap` 的 cursor 委托底层 `java.util.Map` 的 `iterator.remove()` / `Map.Entry.setValue()`（Java `EconomicMapWrap.getEntries()` 返回的匿名 `MapCursor` 重写了 `remove`/`setValue`，非 trait default）。
- **Rust**：trait `UnmodifiableEconomicMap::get_entries` 是 `&self`（对齐 Java `getEntries` 非变更签名），返回的 cursor 借用 map 的不可变引用，无法 mutate 底层 `BTreeMap`。分两类处理：
  - **`BTreeEconomicMapCursor`（计划延后）**：`remove`/`set_value` panic，标注 "deferred to the EconomicMapImpl port"。这是计划性延后——真正的 remove/setValue 语义随完整 `EconomicMapImpl` 移植时补（`EconomicMapImpl` 的 cursor 直接操作内部数组，无借用约束）。
  - **`EconomicMapWrapCursor`（当前缺口）**：Java `EconomicMapWrap` 的 cursor 实际支持 `remove`/`setValue`（委托 `iterator.remove`/`entry.setValue`），因此这是真实功能缺口而非计划延后。Rust 实现因 trait `&self` 约束 + 不引入 `RefCell`/unsafe（避免偏离 Java 存储模型）而暂未支持，`remove`/`set_value` panic 并显式标注 "gap"。修复需引入内部可变性（`RefCell<BTreeMap>`）或破坏 trait `&self` 签名，均偏离 Java 模型；留待 `EconomicMapImpl` 移植或评估内部可变性方案时统一处理。
- **影响**：两类 cursor 的遍历（`advance`/`get_key`/`get_value`）完全可用；仅 `remove`/`set_value` 不可用，但缺口性质不同（`BTreeEconomicMap` 是计划延后，`EconomicMapWrap` 是真实缺口）。

### 3. Null key 拒绝：Rust 类型系统层面处理

- **Java**：`EconomicMapImpl.checkNonNull(key)` 在运行时抛出 `UnsupportedOperationException("null not supported")`。
- **Rust**：Rust 无语言级 null 引用，`check_non_null_key`/`check_non_null_element` 为 no-op 行为锚（保留以对齐 Java 语义路径，但实际无操作）。Null key 在类型层面被排除。
- **影响**：无功能损失；Java 的运行时异常在 Rust 编译期消除。

### 4. compute_if_absent 使用 trait default（需 K: Clone）

- **Java**：`EconomicMap.computeIfAbsent` 是接口 default 方法，使用 `get` + `put`。
- **Rust**：`BTreeEconomicMap` 曾尝试用 BTreeMap entry API 覆盖以避免 `K: Clone`，但 borrow checker（E0515）阻止从 `OccupiedEntry` match 绑定返回引用。改用 trait default（需 `K: Clone`）。
- **影响**：`compute_if_absent` 在 `K: !Clone` 时不可用。完整 `EconomicMapImpl` 移植后将不受此限制。
- **行为**：与 Java 语义一致（absent 或 present-with-null 触发 mapping function）。

### 5. remove_if 位置与绑定

- **Java**：`removeIf` 继承自 `Iterable`，定义在 `UnmodifiableEconomicSet` 上，使用 `Iterator.remove()`。
- **Rust**：`std::iter::Iterator` 无 `remove()` 方法。`remove_if` 保留在 `UnmodifiableEconomicSet` trait 上（对齐 Java 结构），但增加 `Self: EconomicSet<E>` bound，限制仅在可变 set 上可用。
- **影响**：方法位置不变；调用约束更明确（Java 的 unmodifiable set 迭代器 `remove()` 运行时抛异常，Rust 编译期限制）。

### 6. EconomicMapWrap 包装类型

- **Java**：`EconomicMapWrap` 包装 `java.util.Map<K, V>`（V 可为 null）。
- **Rust**：包装 `BTreeMap<K, Option<V>>`，与 crate 内部 null 值表示一致（`Option<V>` 表示 Java 的 nullable V）。
- **影响**：API 一致；内部存储类型为 `Option<V>`。

### 7. 静态工厂方法位置

- **Java**：`EconomicMap`/`EconomicSet` 接口上的 `static` 工厂方法（`create`/`of`/`emptyMap`/`wrapMap`/`emptyCursor`/`emptySet`）。
- **Rust**：Rust 无 trait static 方法，工厂作为 `BTreeEconomicMap`/`BTreeEconomicSet` 的关联函数。Java 工厂委托 `EconomicMapImpl`/`EconomicMapWrap`/`EmptyMap`，Rust 对应委托 `BTreeEconomicMap`/`EconomicMapWrap`/`EmptyMap`。`EconomicSet.create(Iterable<E> c)`（Java 25.1 新增）对应 `BTreeEconomicSet::create_from_slice(&[E])`，内部 `create() + add_all_slice(values)`，对齐 Java `set.addAll(c)` → `addAll(Iterator)` → `add` 委托链。

### 8. 方法重载合并

- **Java**：`addAll(EconomicSet)` / `addAll(Iterable)` / `addAll(Iterator)` 三个重载；`removeAll` 同理。
- **Rust**：Rust 无方法重载，合并为 `add_all_set`（`&dyn EconomicSet`）/ `add_all_slice`（`&[E]`）两个方法。`create(Equivalence, UnmodifiableEconomicSet)` 内部手动遍历 `iterator()` 调用 `add`（对齐 Java `addAll(Iterator)` → `add` 委托）。

### 9. get_or_default 显式生命周期

- **Java**：`V get(K key, V defaultValue)` — 返回值可能为 map 内值或传入的 default。
- **Rust**：`fn get_or_default<'a>(&'a self, key: &K, default_value: &'a V) -> Option<&'a V>` — 显式标注 `default_value` 与返回值共享生命周期 `'a`（覆盖 `&self`），以支持返回 `default_value` 引用。这是 Rust 生命周期系统的必要适配，不影响语义。

### 10. EmptyMap/EmptyCursor/EmptySet 手动 Default impl

- **Java**：`EmptyMap.EMPTY_MAP` 等为单例实例。
- **Rust**：`EmptyMap<K,V>`/`EmptyCursor<K,V>`/`EmptySet<E>` 使用 `PhantomData` 零大小类型。手动实现 `Default`（不使用 `#[derive(Default)]`），避免 `#[derive(Default)]` 要求 `K: Default + V: Default` 的不必要约束。

### 11. Equivalence 子类化扩展

- **Java**：`Equivalence` 为抽象类，`protected Equivalence()` 构造器允许子类化自定义策略。
- **Rust**：`Equivalence` 为 `enum`（Copy + Clone + Debug + PartialEq + Eq + Hash），不支持子类化。三种预定义策略（DEFAULT/IDENTITY/IDENTITY_WITH_SYSTEM_HASHCODE）作为枚举变体。完整 `EconomicMapImpl` 移植时可重新引入开放策略抽象。

### 12. Equivalence.hashCode 算法差异（不阻断，待 EconomicMapImpl 对齐）

- **Java**：`Equivalence.hashCode(o)` 对 `DEFAULT`/`IDENTITY` 调用 `o.hashCode()`，对 `IDENTITY_WITH_SYSTEM_HASHCODE` 调用 `System.identityHashCode(o)`（均为 32-bit signed `int`）。
- **Rust**：`Equivalence::hash_code` 用 `std::collections::hash_map::DefaultHasher`（SipHash）折叠到 `i32`（`DEFAULT`/`IDENTITY`），或对象地址转 `i32`（`IDENTITY_WITH_SYSTEM_HASHCODE`）。算法与 Java 的 `hashCode()`/`System.identityHashCode` 不等价（`DefaultHasher` 是 SipHash-1-3，Java `hashCode` 是各类型自定义的 32-bit 哈希；地址转 `i32` 也非 `identityHashCode` 的真实算法）。
- **影响**：当前 `BTreeMap`/`BTreeSet` 暂替不依赖哈希（用 `Ord` 排序），`Equivalence::hash_code` 仅作 API 形态镜像，无实际调用路径，不影响行为。完整 `EconomicMapImpl` 移植后（依赖哈希分桶），需对齐 Java 哈希算法（可能需为每种键类型提供 Java 兼容的 `hashCode` 实现，或引入 `IdentityHashCode` trait 抽象）。

## 自验结果

- `cargo test -p rustci-collections`：**81 passed, 0 failed**
- `cargo fmt -p rustci-collections -- --check`：**通过**
- `cargo build -p rustci-collections`：**通过**
- `cargo clippy -p rustci-collections --all-targets`：**通过**（无 warning）
- `cargo build --workspace`：**通过**（workspace 全部 crate 编译成功）
