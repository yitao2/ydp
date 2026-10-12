# ydp

`ydp` is a strongly typed, async-only Chrome DevTools Protocol client for Rust.
It combines generated bindings from a fixed Chromium protocol snapshot with the
bounded `ydp-core` communication runtime.

The crate handles protocol commands, responses, events, flattened sessions, and
WebSocket/TLS communication. Browser launching and higher-level browser
automation semantics belong in applications or automation libraries.

This is an early alpha release. APIs and the bundled protocol snapshot may
change before the first stable release.

Licensed under the [MIT License](LICENSE). Generated bindings also retain the
upstream Chromium protocol notice in [LICENSE.protocol](LICENSE.protocol).
