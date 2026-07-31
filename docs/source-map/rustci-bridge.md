# rustci-bridge（Rust cdylib 对接层）

## 源位置：无 Java 源；Rust 原生 cdylib（产物 `librustci.so` / `librustci.dylib`），导出 **7 个 C ABI 符号** 供 HotSpot/宿主 JNI 调用。参考 `jdk.vm.ci.hotspot.CompilerToVM`（132 native）的对接需求。

## 状态：未开始

## 说明

RustCI 与宿主 VM 的桥接层：将 Rust 实现的 JVMCI/Graal 子集以 cdylib 形式暴露给 HotSpot，承接 `CompilerToVM` 等 native 入口。本期仅建框架；开工时由子代理与 `jdk-vm-ci-hotspot.md` 协同，确定 7 符号的签名（Java 侧 `native` 声明 ↔ Rust `#[no_mangle] extern "C"` ↔ C 头文件）后回填下表。

## 映射表

| 桥接符号（C ABI 名） | 对应 Java native 声明（全限定名.方法） | 对应 Rust 文件 | 状态 | 负责人 |
|---|---|---|---|---|
