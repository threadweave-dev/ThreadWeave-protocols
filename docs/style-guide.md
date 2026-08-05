# Protobuf Style Guide

- **Packages:** use `threadweave.<domain>.vN`, all lowercase.
- **Files:** use lowercase `snake_case.proto`, grouped by package.
- **Services:** use PascalCase nouns ending in `Service`.
- **RPCs:** use PascalCase verb phrases such as `SubmitTask`.
- **Messages:** use singular PascalCase nouns; request and response messages
  match their RPC names.
- **Fields:** use `lower_snake_case`. Add fields with new numbers only.
- **Enums:** use PascalCase enum names and uppercase values prefixed by the enum
  name. Zero values end in `_UNSPECIFIED`.
- **Identifiers:** use string fields ending in `_id`; do not embed storage or
  language-specific identifier representations.
- **Timestamps:** use `google.protobuf.Timestamp`; use an `_at` suffix for event
  times and `observed_at` for observation times.
- **Durations:** use `google.protobuf.Duration`, not integer units or strings.
- **Bytes:** use `bytes` only for intentionally opaque or externally serialized
  content. Pair serialized payloads with an explicit serialization format.

Imports are rooted at the `proto` module directory. Public services, RPCs,
messages, enums, and important fields require concise comments. Contracts must
remain transport-focused; behavioral rules belong in RFCs. Do not add REST,
authentication, or authorization annotations without an approved protocol
design.
