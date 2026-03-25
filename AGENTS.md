# AGENTS.md

Guidance for coding agents working in `D:\GITHUB\ALVR`.

## Scope

- This repository is a Rust workspace with most crates under `alvr/*`.
- The primary developer entrypoint is `cargo xtask`, not ad hoc shell scripts.
- Final artifacts are often assembled by xtask into `build/`, so plain `cargo build` is not always enough.
- The workspace uses Rust edition 2024 and MSRV Rust 1.88.
- `openvr/` is a git submodule; some SDK/vendor trees are also checked in.

## Repo Layout

- Workspace root: `Cargo.toml`
- Cargo alias: `.cargo/config.toml` defines `cargo xtask = cargo run -p alvr_xtask --`
- Build tooling: `alvr/xtask`
- Shared crates: `alvr/common`, `alvr/session`, `alvr/packets`, `alvr/filesystem`
- Desktop apps: `alvr/dashboard`, `alvr/launcher`
- Server crates: `alvr/server_core`, `alvr/server_openvr`, `alvr/server_io`
- Client crates: `alvr/client_core`, `alvr/client_openxr`, `alvr/client_mock`
- Linux graphics helper: `alvr/vulkan_layer`
- Wiki build docs: `wiki/Building-From-Source.md`
- Style guide: `CONTRIBUTING.md`

## Cursor / Copilot Rules

- No `.cursorrules` file was found.
- No `.cursor/rules/` directory was found.
- No `.github/copilot-instructions.md` file was found.

## Setup Commands

- Clone with submodules: `git clone --recurse-submodules https://github.com/alvr-org/ALVR.git`
- If already cloned: `git submodule update --init --checkout --recursive`
- Show xtask help: `cargo xtask --help`
- Clean build artifacts and downloaded deps: `cargo xtask clean`

## Build Commands

- Prepare streamer dependencies on Windows: `cargo xtask prepare-deps --platform windows`
- Prepare streamer dependencies on Linux: `cargo xtask prepare-deps --platform linux --no-nvidia`
- Prepare Android dependencies: `cargo xtask prepare-deps --platform android`
- Build streamer package layout: `cargo xtask build-streamer --release`
- Build launcher package layout: `cargo xtask build-launcher --release`
- Build Android client package layout: `cargo xtask build-client --release`
- Run dashboard after build: `cargo xtask run-streamer`
- Run launcher after build: `cargo xtask run-launcher`

## Lint And Format Commands

- Format everything: `cargo xtask format`
- Check formatting only: `cargo xtask check-format`
- Run local clippy rules: `cargo xtask clippy`
- Run CI-grade clippy that fails on warnings: `cargo xtask clippy --ci`
- Plain Cargo fmt still works for Rust-only changes: `cargo fmt --all`
- Plain Cargo clippy is useful for quick iteration on a crate: `cargo clippy -p alvr_session`

## Test Commands

- Main CI test target today: `cargo test -p alvr_session`
- Run all tests in one crate: `cargo test -p alvr_session`
- Run one exact unit test: `cargo test -p alvr_session test_session_to_settings`
- Run one exact test with output: `cargo test -p alvr_session test_session_to_settings -- --nocapture`
- Run one test module pattern: `cargo test -p alvr_session session_extrapolation`
- Run ignored or special lib tests with Cargo flags as usual: `cargo test -p alvr_session -- --ignored`
- If adding tests to another crate, prefer `cargo test -p <crate_name>` over workspace-wide test runs.

## What CI Checks

- Windows and Linux PR CI run dependency preparation plus `cargo xtask clippy --ci`.
- macOS PR CI currently runs plain `cargo clippy`.
- PR CI runs `cargo test -p alvr_session`.
- PR CI runs `cargo xtask check-format`.
- PR CI also builds the Android client.
- Merge queue additionally runs MSRV checks and license checks.

## Build System Notes

- Prefer xtask for anything that needs packaged outputs, copied resources, or external dependencies.
- `cargo xtask build-streamer` builds more than one crate and copies binaries/resources into `build/alvr_streamer_<os>`.
- `cargo xtask build-launcher` copies the launcher executable into `build/alvr_launcher_<os>`.
- Linux streamer builds also assemble the compositor wrapper, Vulkan layer, firewall helpers, and manifests.

## Rust Style Rules

- Follow `CONTRIBUTING.md` first when in doubt.
- Use `cargo fmt --all`; do not hand-format against rustfmt.
- Keep imports alphabetized.
- Group shared import prefixes with braces.
- Do not leave blank lines between import lines, except between private and public import blocks.
- Typical file order is: private imports, public imports, FFI import blocks, private constants, public constants, private structs, public structs, private functions, public functions.
- Keep `mod ...;` declarations near the top of the file.
- Preserve existing `pub use` re-export patterns, especially in `alvr/common`.

## Naming Conventions

- Respect standard Rust naming conventions.
- Prefer descriptive names; avoid abbreviations when practical.
- Do not encode type or scope into names unless it clarifies a real ambiguity.
- `maybe_` is acceptable for local `Option` or `Result` values when it improves readability.
- Use `_dir` for directories, `_path` for file paths, and `_fname` for file names when both appear in the same context.
- Shadowing is acceptable and is explicitly encouraged by the repo style guide.

## Types And Data Modeling

- Prefer enums and structured types over sets of booleans that can represent invalid states.
- Extract arbitrary literals into named constants, ideally with semantic types like `Duration` or `PathBuf` when appropriate.
- Favor existing shared types from `alvr_common`, `alvr_packets`, and `alvr_session` instead of redefining equivalents.
- Keep cross-platform differences behind `cfg(...)` gates rather than runtime flags when possible.

## Error Handling

- Prefer fallible APIs returning `Result` over panicking.
- This codebase commonly uses `anyhow::{Result, Context, bail}` or `alvr_common::anyhow::Result`.
- Use `bail!` and `Context` for operational errors; do not silently discard meaningful failures.
- `panic!()` is discouraged except in tests, build scripts, or truly unrecoverable internal conditions.
- Prefer `unreachable!()` for impossible exhaustive branches.
- Prefer `.get()` over raw indexing unless you are certain indexing is safe.
- `unwrap()` is discouraged in production code, though build scripts and FFI-heavy areas sometimes use it.
- If you must rely on `unwrap()` or raw indexing in nontrivial code, add a short safety rationale comment.

## Comments And Readability

- Add comments for non-obvious intent, invariants, protocol quirks, timing assumptions, or unsafe/FFI behavior.
- Do not add comments that merely restate syntax or obvious library behavior.
- Use whitespace to separate logical blocks inside long functions.
- Prefer small groups of related statements over a wall of declarations at the top of a function.

## Unsafe, FFI, And Platform Code

- This repository has substantial FFI and platform-specific code around OpenVR, OpenXR, graphics, JNI, and C APIs.
- Keep unsafe blocks tight and localized.
- Match the surrounding style for lint allowances on generated bindings or FFI shims.
- Do not remove existing `#[allow(...)]` or `#[expect(...)]` attributes unless you verify they are no longer needed.
- Preserve platform gates and build-script assumptions; many crates are intentionally OS-specific.

## C And C++ Rules

- Rust formatting is handled by rustfmt; C/C++ formatting is handled by `clang-format` via xtask.
- `.clang-format` uses WebKit base style, 4-space indentation, attached braces, and a 100-column limit.
- `.editorconfig` enforces LF endings and final newlines for C/C++ files.
- Avoid drive-by reformatting in vendored or excluded code.
- `alvr/xtask/src/format.rs` excludes some third-party and NVENC-related files from formatting checks; preserve that intent.

## Generated And Vendored Files

- Do not manually edit generated files emitted from `OUT_DIR` such as `bindings.rs`, `layer_bindings.rs`, or `openvr_property_keys.rs`.
- Treat cbindgen outputs as generated artifacts.
- Avoid unnecessary edits under `openvr/`; it is a submodule.
- Avoid unnecessary edits under checked-in SDK/vendor trees like `yvr_openxr_mobile_sdk_2.0.0` and `__MACOSX`.
- If you must change generated inputs, edit the source templates, build scripts, or schema definitions instead.

## Agent Workflow Advice

- Before changing build logic, read the relevant xtask implementation in `alvr/xtask/src`.
- Before changing packaged paths, read `alvr/filesystem/src/lib.rs`.
- Before changing style-sensitive code, skim `CONTRIBUTING.md` and nearby files in the same crate.
- Do not normalize naming, imports, or cfg layout unless the touched file already needs it.
- When adding tests, start with crate-local tests and document the exact command used.
- After Rust code changes, default verification is `cargo xtask check-format`, `cargo xtask clippy`, and a targeted `cargo test -p <crate>` when feasible.
