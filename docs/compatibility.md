# Compatibility

ThreadWeave contracts prioritize compatibility across independently released
core and SDK implementations.

## Compatible changes

Normally compatible changes include adding an optional or repeated field with a
safe default, adding a new message, and adding a new RPC when clients are not
required to use it. New enum values require care: receivers must preserve or
handle unknown numeric values safely.

## Breaking changes

Breaking changes include removing or renaming fields, changing field numbers,
changing incompatible field types, moving types between packages, changing
oneof membership, renaming generated API symbols, or altering service method
signatures. Such changes require a new versioned package.

Field numbers and enum numeric values are permanent once released. Removed
fields and enum values must reserve both their number and name. An enum's zero
value must remain an explicit `_UNSPECIFIED` value.

Buf's `FILE` breaking policy protects both wire compatibility and common
generated source APIs. Passing automated checks is necessary but does not prove
semantic compatibility; the relevant RFC remains authoritative.

The core and SDKs need not share release numbers. Each implementation must
declare the tagged protocol release and package versions it supports. Two
implementations are expected to interoperate only where those supported
versions overlap.
