# jdk.graal.compiler.libgraal

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler.libgraal/src/jdk/graal/compiler/libgraal/`（**独立模块** `jdk.graal.compiler.libgraal`，非 `jdk.graal.compiler` 子包）

## 状态：未开始

## 说明

libgraal 原生镜像入口：将 Graal 编译进共享库（`LibGraal`、`LibGraalClassLoader`、`LibGraalSupportImpl`、`GetCompilerConfig` 等），与 SVM/SubstrateVM 协作，供 HotSpot 以库形式调用。独立顶层模块。第三期及以后范围，本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class 全限定名后回填。与 [rustci-bridge.md](rustci-bridge.md) 协同。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
