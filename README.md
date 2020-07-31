# Minimal fmax/fmin linker problem example
Run `cargo build` to get the following: (note that you need to have the toolchain for target `thumbv7em-none-eabihf` installed)

```
error: linking with `rust-lld` failed: exit code: 1
  |
  = note: "rust-lld" "-flavor" "gnu" "-L" "/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/fmin_test-901498e0bfdafd74.4mtbugsqy689bp5t.rcgu.o" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/fmin_test-901498e0bfdafd74.91k32w8pgqyjis.rcgu.o" "-o" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/fmin_test-901498e0bfdafd74" "--gc-sections" "-L" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps" "-L" "/home/wouter/personal/fmin-test/target/debug/deps" "-L" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/build/cortex-m-rt-0bfb9154dd9630e3/out" "-L" "/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib" "-Bstatic" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/libcortex_m_rt-7fab94e75277efdb.rlib" "/home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/libr0-393423e46585ef52.rlib" "/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib/librustc_std_workspace_core-a90c1e246dd7cbb9.rlib" "/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib/libcore-e10163bfb70adfb4.rlib" "/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib/libcompiler_builtins-48128945b71f1d49.rlib" "-Tlink.x" "-Bdynamic"
  = note: rust-lld: error: undefined symbol: fminf
          >>> referenced by f32.rs:603 (/home/wouter/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/num/f32.rs:603)
          >>>               /home/wouter/personal/fmin-test/target/thumbv7em-none-eabihf/debug/deps/fmin_test-901498e0bfdafd74.4mtbugsqy689bp5t.rcgu.o:(core::f32::_$LT$impl$u20$f32$GT$::min::h4fbc1895c0cb40eb)
```