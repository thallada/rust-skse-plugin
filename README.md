# Rust SKSE Plugin

This is my mostly failed attempt at creating a fully-Rust [SKSE (Skyrim Script Extender)](https://skse.silverlock.org/) plugin.

## Build and Install

1. Have Skyrim Special Edition version 1.5.97 or later installed with SKSE build 2.0.17 installed.
2. Checkout the repo and `cd rust-skse-plugin`.
3. Run `cargo build`.
4. Copy `target/debug/RustSKSEPlugin.dll` to `<Skyrim Special Edition install folder>\Data\SKSE\Plugins\`.
5. Start Skyrim Special Edition by running `skse_loader.exe`.
6. Open `<Your Documents Directory>\My Games\Skyrim Special Edition\SKSE\skse64.log`, and you should see this plugin being loaded:
    ```
    checking plugin E:\Program Files (x86)\Steam\steamapps\common\Skyrim Special Edition\Data\SKSE\Plugins\\RustSKSEPlugin.dll
    plugin E:\Program Files (x86)\Steam\steamapps\common\Skyrim Special Edition\Data\SKSE\Plugins\\RustSKSEPlugin.dll (00000001 My Rust SKSE Plugin 00000001) loaded correctly
    ```
7. Open `<Your Documents Directory>\My Games\Skyrim Special Edition\SKSE\RustSKSEPlugin.log`, and you should see some logs from the Rust SKSE plugin:
    ```
    [00:00:00.000] (8c4) INFO   SKSEPlugin_Query begin
    [00:00:00.000] (8c4) INFO   SKSEPlugin_Query successful
    [00:00:00.000] (8c4) INFO   SKSEPlugin_Load begin
    [00:00:00.000] (8c4) INFO   queryInterfaceFunc: 0x7ffea09c695a
    [00:00:00.000] (8c4) INFO   queryInterface: 0x7ffea0e4eba8
    [00:00:00.000] (8c4) INFO   papyrusInterface: 0x7ffea0e4eba8
    [00:00:00.000] (8c4) INFO   papyrusRegister: 0x7ffea09d2011
    [00:00:00.000] (8c4) INFO   SKSEPlugin_Load successful
    [00:00:01.740] (7838) INFO   RegisterFuncs begin
    [00:00:01.740] (7838) INFO   a_registry: 0x19510f93780
    [00:00:01.740] (7838) INFO   registerFunction: 0x7ff7c85384f0
    [00:00:01.740] (7838) INFO   nativeFunction: 0x26ebebf108
    [00:00:01.740] (7838) INFO   RegisterFuncs successful
    ```

## Where I got stuck

I was trying to replicate [Ryan-rsm-McKenzie's Native SKSE64 Papyrus Interface Implementation example](https://gist.github.com/Ryan-rsm-McKenzie/cabb89a80abb09663a1288cafddd21e6) in Rust via its FFI to C. I was able to successfully register the plugin with SKSE and even acquire a reference to the `PapyrusInterface`.

However, it seems like the code needs to call a C++ constructor (e.g. `new NativeFunction0<StaticFunctionTag, BSFixedString>("HelloWorld", "MyClass", HelloWorld, a_registry)`) in order to register a new native Papyrus function. Unfortunately, calling a C++ constructor requires FFI with C++, and [Rust does not support FFI with C++](https://stackoverflow.com/a/45540511).

## Failed attempt at using cpp crate

The [`cpp`](https://docs.rs/cpp/0.5.4/cpp/) crate may provide a way to interface with the SKSE C++ classes, but I'm too inexperienced with C++ to figure out how to compile skse64 in the `build.rs`.

I was able to get `CommonLibSSE` to compile, but I could not figure out how to convert the abstract `RE::BSScript::IVirtualMachine` class that the `RegisterFuncs` function recieves into something useable on the Rust side. That work is not included in the source, but to get that to work you will need to first follow the [CommonLibSSE setup](https://github.com/Ryan-rsm-McKenzie/CommonLibSSE/wiki/Getting-Started#building-your-first-plugin) and make sure everything builds in Visual Studio first.

Then checkout this repo alongside the `CommonLibSSE` folder inside `skse64`. Add the `cpp` and `cpp_build` crates to `Cargo.toml` and create this `build.rs` file at the root of the project:

```rust
extern crate cpp_build;

fn main() {
    let include_path = r#"<path to skse src>\skse64\CommonLibSSE\include"#;
    let lib_path = r#"<path to skse src>\skse64\x64\Debug"#;
    let vcpkg_include_path = r#"<path to vcpkg folder>\installed\x64-windows-custom\include"#;
    cpp_build::Config::new().include(include_path).include(vcpkg_include_path).flag("/std:c++17").build("src/lib.rs");
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=CommonLibSSE");
}
```

And, add this to `src/lib.rs`:

```rust
#[macro_use]
extern crate cpp;

cpp!{{
    #include "RE/Skyrim.h"
    #include "REL/Relocation.h"
    #include "SKSE/SKSE.h"
}}
```

Now you can replace the `SKSEPapyrusInterface` struct with something like:
```rust
cpp_class!(pub unsafe struct PapyrusInterface as "SKSE::PapyrusInterface");
```

## What I'm probably going to do instead

Abandon my dream of a pure Rust SKSE plugin and just write a normal C++ one (with [CommonLibSSE](https://github.com/Ryan-rsm-McKenzie/CommonLibSSE)) which will execute functions exported from a Rust dll (with [cbindgen](https://crates.io/crates/cbindgen)) inside a native Papyrus function callback. This requires the user to place the Rust-generated dll file in their Skyrim install directory in addition to placing the C++ generated SKSE plugin dll in the SKSE plugins directory.