# ADR 0001: Rust core with a Kotlin Android UI

- Status: Accepted
- Date: 2026-11-09

## Context

Dexsentry needs to inspect APK and DEX binary data on Android and desktop. The parsing code must share one report model and remain isolated from UI concerns. Android users expect a native interface and platform file selection.

## Decision

Implement parsing, rules, report generation, and the CLI in a Rust workspace. Use Kotlin and Jetpack Compose for Android. Keep the UniFFI crate as a thin adapter with its own foreign-facing records, enums, and errors.

## Consequences

- Android and the CLI call the same Rust core.
- Rust domain types do not depend on UniFFI.
- Generated Kotlin and ABI-specific native libraries are build outputs.
- Kotlin owns app navigation, state, and Android permissions/file selection.
- The current scaffold has no APK/DEX parsing or malware detection logic.
