# jdk.graal.compiler.util.collections

## 源位置：`/opt/graal/sdk/src/org.graalvm.collections/src/org/graalvm/collections/`（**sdk 模块**，包名 `org.graalvm.collections`；非 compiler 子目录）

## 状态：未开始

## 说明

`EconomicMap` / `EconomicSet` 等 nil-dependent 集合类型，是 graph / nodes / util.json 的基础数据结构。注意：源在 **sdk 模块**（`org.graalvm.collections` 包），不在 compiler；本 MD 作为该集合层移植的统一索引。第二期范围，本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class（EconomicMap、EconomicSet、MapCursor、Pair、Equivalence、ListBuilder 等）全限定名后回填。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
