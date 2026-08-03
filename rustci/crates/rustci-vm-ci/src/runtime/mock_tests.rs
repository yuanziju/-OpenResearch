// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
/*
 * Copyright (c) 2024, 2025, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy has been included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have
 * questions.
 */

//! `runtime` 模块最小 mock 测试：验证核心 trait/struct 构造、常量与行为。

use crate::code::architecture::Architecture;
use crate::code::compilation_request::CompilationRequest;
use crate::code::compilation_request_result::CompilationRequestResult;
use crate::runtime::jvmci::JVMCI;
use crate::runtime::jvmci_backend::JVMCIBackend;
use crate::runtime::jvmci_compiler::{JVMCICompiler, INVOCATION_ENTRY_BCI};
use crate::runtime::jvmci_compiler_factory::JVMCICompilerFactory;
use crate::runtime::jvmci_runtime::JVMCIRuntime;

struct MockCompiler;

impl JVMCICompiler for MockCompiler {
    fn compile_method(&self, _request: &CompilationRequest) -> Box<dyn CompilationRequestResult> {
        unimplemented!()
    }
}

struct MockCompilerFactory;

impl JVMCICompilerFactory for MockCompilerFactory {
    fn get_compiler_name(&self) -> String {
        "mock".to_string()
    }

    fn create_compiler(&self, _runtime: &dyn JVMCIRuntime) -> Box<dyn JVMCICompiler> {
        Box::new(MockCompiler)
    }
}

struct MockRuntime;

impl JVMCIRuntime for MockRuntime {
    fn get_compiler(&self) -> Box<dyn JVMCICompiler> {
        Box::new(MockCompiler)
    }

    fn get_host_jvmci_backend(&self) -> &JVMCIBackend {
        unimplemented!()
    }

    fn get_jvmci_backend(&self, _arch: &Architecture) -> Option<&JVMCIBackend> {
        None
    }
}

fn mock_initialize_runtime() -> Box<dyn JVMCIRuntime> {
    Box::new(MockRuntime)
}

#[test]
fn jvmci_compiler_constants_and_defaults() {
    assert_eq!(INVOCATION_ENTRY_BCI, -1);
    let compiler = MockCompiler;
    assert!(compiler.is_gc_supported(0));
    assert!(!compiler.is_intrinsic_supported(0));
}

#[test]
fn jvmci_compiler_factory_defaults() {
    let factory = MockCompilerFactory;
    assert_eq!(factory.get_compiler_name(), "mock");
    factory.on_selection();
    let mut buf: Vec<u8> = Vec::new();
    factory.print_properties(&mut buf);
    assert!(buf.is_empty());
    let runtime = MockRuntime;
    let _compiler = factory.create_compiler(&runtime);
}

#[test]
fn jvmci_get_runtime_caches_singleton() {
    let _ = JVMCI::register_initialize_runtime(mock_initialize_runtime);
    let rt1 = JVMCI::get_runtime();
    let rt2 = JVMCI::get_runtime();
    assert!(std::ptr::eq(rt1, rt2));
}

#[test]
fn jvmci_initialize_is_noop() {
    JVMCI::initialize();
}
