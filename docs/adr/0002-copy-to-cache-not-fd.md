# ADR 0002: Copy selected APKs to cache and pass a path

- Status: Accepted
- Date: 2026-11-09

## Context

Android's Storage Access Framework returns content URIs, and the Rust core currently accepts a filesystem path. Passing a raw file descriptor would require carefully coordinating its lifetime and ownership across the Kotlin/Rust boundary. Reconstructing a Rust `File` with `from_raw_fd` would require `unsafe` code.

## Decision

When file selection is implemented, Kotlin will stream the selected content URI into a uniquely named file under the app's private `cacheDir`, then pass that path to Rust. The core opens the file itself. Kotlin removes the cached copy after analysis or when it is no longer needed.

## Consequences

- The FFI signature stays path-based and contains no raw descriptor ownership rules.
- The app must handle copy failures, cache cleanup, and available storage.
- The current scaffold does not yet implement file selection or copying.
