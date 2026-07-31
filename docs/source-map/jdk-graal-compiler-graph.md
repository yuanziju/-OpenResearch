# jdk.graal.compiler.graph

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/graph/`

## 状态：未开始

## 说明

编译器图数据结构：`Node`、`Graph`、`NodeClass`、`Block`、`Successor`/`Input` 边模型、`NodeMap` 等，是 nodes / core / phases 的基础。依赖 util.collections / nodeinfo / debug。第三期范围，本目录顶层约 30 个 .java；本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class 全限定名后回填。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
