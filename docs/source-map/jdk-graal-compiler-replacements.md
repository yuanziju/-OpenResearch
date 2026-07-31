# jdk.graal.compiler.replacements

## 源位置：`/opt/graal/compiler/src/jdk.graal.compiler/src/jdk/graal/compiler/replacements/`

## 状态：未开始

## 说明

字节码替换 / intrinsic 注入：`@MethodSubstitution`、`Snippet`、`Replacement`、`ReplacementsImpl` 等，将特定方法替换为编译期已知实现。依赖 nodes / graph / phases / jdk.vm.ci.meta。第三期范围，本目录顶层约 38 个 .java；本期仅建框架，不预填条目；开工时由子代理列出 public 接口与 final class 全限定名后回填。

## 映射表

| Java 类（全限定名） | 职责 | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
