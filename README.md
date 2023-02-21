```
CARGO_BUILD_RUSTFLAGS=-Zsanitizer=address rustup run nightly cargo test -Zbuild-std --target x86_64-unknown-linux-gnu
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/x86_64-unknown-linux-gnu/debug/deps/rules_rust_asan-4fb21504aab84b3b)

running 1 test
test test_main ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
bazel test //... --@rules_rust//:extra_exec_rustc_flags=-Zsanitizer=address --@rules_rust//rust/toolchain/channel=nightly
INFO: Invocation ID: 256c49f3-7989-4d11-a7c2-46294a46da6e
INFO: Build options --copt and --linkopt have changed, discarding analysis cache.
INFO: Analyzed target //:main_test (0 packages loaded, 2205 targets configured).
INFO: Found 1 test target...
ERROR: /usr/local/google/home/aaronyu/.cache/bazel/_bazel_aaronyu/be28df46df27005c77b80d2d9bce3568/external/crate_index__proc-macro-error-1.0.4/BUILD.bazel:22:13: Compiling Rust rlib proc_macro_error v1.0.4 (26 files) [for tool] failed: (Exit 1): process_wrapper failed: error executing command (from target @crate_index__proc-macro-error-1.0.4//:proc_macro_error) bazel-out/k8-opt-exec-2B5CBBC6/bin/external/rules_rust/util/process_wrapper/process_wrapper --env-file ... (remaining 50 arguments skipped)

Use --sandbox_debug to see verbose messages from the sandbox and retain the sandbox build root for debugging
error: /usr/local/google/home/aaronyu/.cache/bazel/_bazel_aaronyu/be28df46df27005c77b80d2d9bce3568/execroot/__main__/bazel-out/k8-opt-exec-2B5CBBC6/bin/external/crate_index__proc-macro-error-attr-1.0.4/libproc_macro_error_attr-1329550019.so: undefined symbol: __asan_option_detect_stack_use_after_return
   --> external/crate_index__proc-macro-error-1.0.4/src/lib.rs:284:9
    |
284 | pub use proc_macro_error_attr::proc_macro_error;
    |         ^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Target //:main_test failed to build
Use --verbose_failures to see the command lines of failed build steps.
INFO: Elapsed time: 0.597s, Critical Path: 0.15s
INFO: 7 processes: 5 internal, 2 linux-sandbox.
FAILED: Build did NOT complete successfully
//:main_test                                                    FAILED TO BUILD

Executed 0 out of 1 test: 1 fails to build.
```
