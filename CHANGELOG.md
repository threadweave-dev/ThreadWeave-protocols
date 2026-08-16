# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.1.2] - 2026-08-12

### Fixed

- Allow the release workflow to package the protocol sources assembled in the
  Rust crate after checkout.

## [1.1.1] - 2026-08-12

### Added

- Add a unified Rust crate providing Prost messages and Tonic gRPC bindings.
- Publish the Rust crate to crates.io as part of tagged protocol releases.

## [1.1.0] - 2026-08-12

### Added

- Add immutable task identities, command outcomes, execution attempts, cancellation,
  pagination, and causal submission fields from RFC010.
- Add extensible resources and scheduling reservations from RFC011.
- Add worker registration, assignment, and execution reporting from RFC012.
- Add participant incarnations, leases, and membership revisions from RFC013.
- Add broker envelopes, artifact locations and residency, and event replay metadata
  from RFC020, RFC021, and RFC030.

## [1.0.0] - 2026-08-05

### Added

- Initial versioned ThreadWeave protocol contracts.
- Buf lint, generation, and breaking-change configuration.
- Protocol documentation and CI validation.

## [1.2.0] - 2026-08-15

### Added

- Add unary long-poll execution acquisition for language runtime workers.
- Reuse assignment and execution reporting messages for worker lifecycle updates.

## [1.3.0] - 2026-08-16
### Runtime sessions

- Added the bidirectional `RuntimeSession` streaming RPC.
- Added extensible `WorkerCommand` and `RuntimeEvent` envelopes using protobuf `oneof`.
- Added assignment and cancellation commands.
- Added runtime-ready, lifecycle, progress, metrics, failure, and heartbeat events.
- Preserved assignment IDs, execution IDs, sequence numbers, `JobResult`, and `Error`.
- Removed the obsolete `AcquireExecution` long-polling RPC.
- Added Python gRPC binding generation.

### Breaking changes

Language runtimes must replace `AcquireExecution` polling with a persistent
`RuntimeSession` stream.

`ReportExecution` remains available for Worker-to-Core lifecycle forwarding.