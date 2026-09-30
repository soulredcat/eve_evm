# Public runtime

Planning only; the runtime is not implemented yet.

Permissionless RPC/P2P node with bounded transaction ingestion, verified state queries, snapshot/delta bootstrap, peer data distribution and explicit readiness. It has no voting power unless a separately registered validator runtime is co-located.

A public package must build without master implementation or secrets. RAM-only non-voting replicas must reverify after restart and report unavailable/pruned data honestly. State sources may be peers or master, but source identity is not a trust anchor.

Read plans 10, 15, 18–20 and 22. Public requests must not starve validator consensus when roles share a machine.
