# ydp-core

`ydp-core` is the bounded, async-only communication runtime used by `ydp` and
custom CDP-compatible protocol bindings.

It provides request correlation, flattened session routing, bounded event
subscriptions, cancellation-aware local lifetimes, and WebSocket/TLS transport.
It does not launch browsers or implement browser automation concepts such as
pages, frames, locators, retries, reconnection, or replay.

This is an early alpha release. APIs may change before the first stable release.

Licensed under the [MIT License](LICENSE).
