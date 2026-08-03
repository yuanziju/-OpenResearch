/*
 * Copyright (c) 2024, 2026, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

// SPDX-License-Identifier: GPL-2.0-with-classpath-exception
//
// Rust mirror of `jdk.graal.compiler.core.CompilationWrapper`.
// Abstract class → struct with abstract methods via trait objects.

use std::collections::HashMap;
use std::fmt::Debug;

/// Actions to take upon an exception being raised during compilation.
/// The actions are in ascending order of verbosity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExceptionAction {
    /// Print nothing to the console.
    Silent = 0,
    /// Print a stack trace to the console.
    Print = 1,
    /// An exception causes the compilation to be retried with extra diagnostics enabled.
    Diagnose = 2,
    /// Same as Diagnose except that the VM process is exited after retrying.
    ExitVM = 3,
}

impl ExceptionAction {
    /// Gets the action that is one level less verbose than this action,
    /// bottoming out at the least verbose action.
    pub fn quieter(self) -> Self {
        match self {
            Self::Silent => Self::Silent,
            Self::Print => Self::Silent,
            Self::Diagnose => Self::Print,
            Self::ExitVM => Self::Diagnose,
        }
    }
}

/// Represents a compilation failure.
///
/// Mirrors `CompilationWrapper.Failure` inner class in Java.
/// Contains the cause of the failure and provides a handle() method.
#[derive(Debug)]
pub struct CompilationFailure {
    /// The cause of the failure.
    pub cause: String,
    /// Whether debug info is available.
    pub debug: bool,
}

impl CompilationFailure {
    /// Creates a new `CompilationFailure`.
    pub fn new(cause: String, debug: bool) -> Self {
        CompilationFailure { cause, debug }
    }

    /// Handles the compilation failure.
    ///
    /// Mirrors `Failure.handle(boolean)` in Java.
    /// If silent, suppresses all logging and dumping.
    pub fn handle(&self, silent: bool) -> String {
        if silent {
            format!("Silent failure: {}", self.cause)
        } else {
            format!("Compilation failure: {}", self.cause)
        }
    }
}

/// Callback trait for CompilationWrapper — subclasses implement these abstract methods.
pub trait CompilationWrapperCallback<T> {
    /// Perform the compilation wrapped by this object.
    ///
    /// Mirrors `performCompilation(DebugContext)` in Java.
    fn perform_compilation(&self) -> Result<T, String>;

    /// Gets a value that represents the input to the compilation.
    fn to_string(&self) -> String;

    /// Handles an uncaught exception.
    ///
    /// Mirrors `handleException(Throwable)` in Java.
    fn handle_exception(&self, cause: &str) -> T;

    /// Creates the DebugContext to use when retrying a compilation.
    ///
    /// Mirrors `createRetryDebugContext(DebugContext, OptionValues, PrintStream)` in Java.
    fn create_retry_debug_context(&self) -> bool {
        false
    }

    /// Parses options to be used for retry compilations.
    ///
    /// Mirrors `parseRetryOptions(String[], EconomicMap)` in Java.
    fn parse_retry_options(&self, options: &[String]) -> HashMap<String, String> {
        let _ = options;
        HashMap::new()
    }

    /// Calls System.exit in the runtime embedding Graal.
    ///
    /// Mirrors `exitHostVM(int)` in Java.
    fn exit_host_vm(&self, _status: i32) {}

    /// Dump any objects for the original failure.
    ///
    /// Mirrors `dumpOnError(DebugContext, Throwable)` in Java.
    fn dump_on_error(&self, _cause: &str) {}
}

/// Wrapper for a compilation that centralizes what action to take based on
/// `CompilationBailoutAsFailure` and `CompilationFailureAction` when an
/// uncaught exception occurs during compilation.
///
/// Mirrors `abstract class CompilationWrapper<T>` in Java.
/// Contains fields: outputDirectory, problemsHandledPerAction.
pub struct CompilationWrapper<T, C: CompilationWrapperCallback<T>> {
    /// Object used to access a directory for dumping if compilation is re-executed.
    pub output_directory: String,
    /// Map tracking how many problems have been handled per action.
    pub problems_handled_per_action: HashMap<ExceptionAction, i32>,
    /// Callback for compilation-specific operations.
    pub callback: C,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Debug, C: CompilationWrapperCallback<T>> CompilationWrapper<T, C> {
    /// Creates a new `CompilationWrapper`.
    ///
    /// Mirrors the `CompilationWrapper(DiagnosticsOutputDirectory, Map<ExceptionAction, Integer>)` constructor.
    pub fn new(output_directory: String, callback: C) -> Self {
        CompilationWrapper {
            output_directory,
            problems_handled_per_action: HashMap::new(),
            callback,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Entry point — runs the compilation with error handling.
    ///
    /// Mirrors `CompilationWrapper.run(DebugContext)` in Java.
    pub fn run(&self) -> Result<T, String> {
        match self.callback.perform_compilation() {
            Ok(result) => Ok(result),
            Err(cause) => {
                let failure = CompilationFailure::new(cause.clone(), false);
                Ok(self.on_compilation_failure(&failure))
            }
        }
    }

    /// Entry point for handling a compilation failure.
    ///
    /// Mirrors `onCompilationFailure(Failure)` in Java.
    pub fn on_compilation_failure(&self, failure: &CompilationFailure) -> T {
        // In the full implementation, this would call failure.handle(false)
        // which would then call handleFailure with the debug context.
        self.callback.handle_exception(&failure.cause)
    }

    /// Looks up the action to take for a given exception.
    ///
    /// Mirrors `lookupAction(OptionValues, Throwable)` in Java.
    pub fn lookup_action(&self, _cause: &str) -> ExceptionAction {
        // In the full implementation, this looks up CompilationFailureAction option
        // and checks if the cause is a BailoutException with CompilationBailoutAsFailure.
        ExceptionAction::Print
    }

    /// Handles a compilation failure with full diagnostic logic.
    ///
    /// Mirrors `handleFailure(DebugContext, Throwable)` in Java — ~100 lines of complex logic.
    pub fn handle_failure(&self, cause: &str) -> T {
        let action = self.lookup_action(cause);
        let adjusted_action = self.adjust_action(action, cause);

        match adjusted_action {
            ExceptionAction::Silent => {
                self.callback.handle_exception(cause)
            }
            ExceptionAction::Print => {
                // Print stack trace and return
                self.callback.handle_exception(cause)
            }
            ExceptionAction::Diagnose | ExceptionAction::ExitVM => {
                // Perform diagnostic retry
                self.perform_diagnostic_retry(cause, adjusted_action)
            }
        }
    }

    /// Adjusts the action based on problem counts and systemic failure rate.
    ///
    /// Mirrors `adjustAction(OptionValues, ExceptionAction, Throwable)` in Java.
    fn adjust_action(&self, action: ExceptionAction, _cause: &str) -> ExceptionAction {
        let mut adjusted = action;
        if adjusted != ExceptionAction::ExitVM {
            if self.detect_compilation_failure_rate_too_high(_cause) {
                return ExceptionAction::ExitVM;
            }
            while adjusted != ExceptionAction::Silent {
                let count = self.problems_handled_per_action.get(&adjusted).copied().unwrap_or(0);
                if count >= 5 {
                    // MaxCompilationProblemsPerAction default
                    adjusted = adjusted.quieter();
                } else {
                    break;
                }
            }
        }
        adjusted
    }

    /// Detects systemic compilation failure rate too high.
    ///
    /// Mirrors `detectCompilationFailureRateTooHigh(OptionValues, Throwable)` in Java.
    fn detect_compilation_failure_rate_too_high(&self, _cause: &str) -> bool {
        // In the full implementation, this checks the SystemicCompilationFailureRate option,
        // compares failed compilations to total compilations in the current period,
        // and may trigger VM exit if the rate exceeds the threshold.
        false
    }

    /// Performs a diagnostic retry compilation.
    ///
    /// Mirrors `performDiagnosticRetry(DebugContext, OptionValues, Throwable, ExceptionAction)` in Java.
    fn perform_diagnostic_retry(&self, _cause: &str, _action: ExceptionAction) -> T {
        // In the full implementation, this:
        // 1. Creates a dump directory
        // 2. Prints the failure message
        // 3. Creates retry options with diagnostics enabled
        // 4. Re-runs performCompilation with the retry debug context
        // 5. Logs the retry result
        self.callback.handle_exception(_cause)
    }

    /// Checks if the bailout should be considered a non-failure.
    ///
    /// Mirrors `isNonFailureBailout(OptionValues, Throwable)` in Java.
    pub fn is_non_failure_bailout(&self, _cause: &str) -> bool {
        false
    }

    /// Gets the diagnostic output directory.
    pub fn get_output_directory(&self) -> &str {
        &self.output_directory
    }

    /// Gets the map of problems handled per action.
    pub fn get_problems_handled_per_action(&self) -> &HashMap<ExceptionAction, i32> {
        &self.problems_handled_per_action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCallback;

    impl CompilationWrapperCallback<String> for TestCallback {
        fn perform_compilation(&self) -> Result<String, String> {
            Ok("success".to_string())
        }

        fn to_string(&self) -> String {
            "TestCallback".to_string()
        }

        fn handle_exception(&self, cause: &str) -> String {
            format!("handled: {}", cause)
        }
    }

    #[test]
    fn test_exception_action_quieter() {
        assert_eq!(ExceptionAction::Silent.quieter(), ExceptionAction::Silent);
        assert_eq!(ExceptionAction::Print.quieter(), ExceptionAction::Silent);
        assert_eq!(ExceptionAction::Diagnose.quieter(), ExceptionAction::Print);
        assert_eq!(ExceptionAction::ExitVM.quieter(), ExceptionAction::Diagnose);
    }

    #[test]
    fn test_exception_action_ordering() {
        assert!(ExceptionAction::Silent < ExceptionAction::Print);
        assert!(ExceptionAction::Print < ExceptionAction::Diagnose);
        assert!(ExceptionAction::Diagnose < ExceptionAction::ExitVM);
    }

    #[test]
    fn test_compilation_wrapper_run_success() {
        let cb = TestCallback;
        let wrapper = CompilationWrapper::new("/tmp/dumps".to_string(), cb);
        let result = wrapper.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_compilation_failure_new() {
        let failure = CompilationFailure::new("test error".to_string(), true);
        assert_eq!(failure.cause, "test error");
        assert!(failure.debug);
        let result = failure.handle(false);
        assert!(result.contains("test error"));
    }

    #[test]
    fn test_compilation_failure_silent() {
        let failure = CompilationFailure::new("test error".to_string(), true);
        let result = failure.handle(true);
        assert!(result.contains("Silent"));
    }

    #[test]
    fn test_lookup_action() {
        let cb = TestCallback;
        let wrapper = CompilationWrapper::new("/tmp/dumps".to_string(), cb);
        let action = wrapper.lookup_action("test error");
        assert_eq!(action, ExceptionAction::Print);
    }
}