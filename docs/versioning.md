# Versioning

Repository releases follow Semantic Versioning. A major release may contain
breaking wire or generated-API changes, a minor release contains compatible
additions, and a patch release contains documentation or other non-contractual
corrections.

Protobuf packages are independently versioned in their final component, for
example `threadweave.execution.v1`. A repository release can contain several
package versions.

## Evolution within a package

Add a field when the new information is optional to existing senders and
receivers and has a safe default. Use a new, previously unused field number and
do not change the meaning of an existing field. Add enum values only when
unknown values are safe for existing consumers.

Create a new package version when a required semantic change cannot be
represented additively, when generated APIs must change incompatibly, or when
the wire representation must change. Migrations between package versions must
be described by an RFC.

Deprecated fields remain in place with `[deprecated = true]` during a migration.
If a field is later removed, reserve both its number and name. Never reuse or
renumber it. Deprecated packages remain available for the support period stated
by the relevant release policy.
