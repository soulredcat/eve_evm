# Validator execution component

Canonical owner: validator execution. This package contains the deterministic
Shanghai serial reference, signed transaction decoding, receipt/state roots and
EVE fee allocation. Its in-memory reference is not consensus or durable storage.

Public simulation/replay and master auditing may reuse the narrow execution
contract through reproducible source packaging from this canonical component.
Do not create a second handwritten EVM/fee implementation under another role.

The default shared workspace currently supplies build metadata. Complete role
distributions still need standalone manifests/lockfiles and isolated copy/build/run
verification; a compiling component is not a running independent validator.
