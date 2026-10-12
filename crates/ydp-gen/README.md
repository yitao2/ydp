# ydp-gen

`ydp-gen` is the offline, reproducible Rust generator used by `ydp` and custom
CDP-compatible protocol bindings. It accepts pinned JSON or PDL inputs, validates
their hashes and schema semantics, and emits deterministic Rust source files.

Generation performs no network access or online translation. Protocol
descriptions retain their source-language text, while optional reviewed Chinese
supplements can be supplied through the manifest.

This is an early alpha release. Generator configuration and generated Rust APIs
may change before the first stable release.

Licensed under the [MIT License](LICENSE).
