# 04 — Master and Public Node Security

## Security objective

A compromise of a public node must not imply direct compromise of canonical master storage or administrative control.

## Trust zones

```text
Internet
  ↓
Edge / DDoS / Load Balancer
  ↓
Public RPC Nodes
  ↓
Authenticated Internal Gateway
  ↓
Execution / Consensus Network
  ↓
Canonical Master Network
  ↓
Replication / Backup Network
```

## Master node rules

The canonical master should:

- have no public RPC endpoint;
- have no directly reachable public IP service;
- reject arbitrary inbound protocols;
- accept only strict versioned internal messages;
- use mutual authentication between node identities;
- use independent management access;
- keep signing keys outside the general application process where practical;
- treat all messages from public nodes as untrusted input.

## Public node compromise model

Assume an attacker can fully control one public node.

The attacker must still be unable to:

- write canonical state directly;
- access canonical database files;
- invoke unrestricted master administrative APIs;
- forge validator quorum/finality certificates;
- push malformed state deltas past protocol validation;
- pivot through shared credentials.

## Input hardening

All internal protocol inputs need:

- strict maximum sizes;
- bounded decoding;
- version checks;
- replay protection where appropriate;
- signature/authentication validation;
- rate limits;
- checksums/hashes;
- fuzz testing.

## Master availability

If all public nodes fail, canonical durable state should remain recoverable.

Whether the chain is allowed to continue *finalizing* new blocks without validator quorum is a separate consensus policy and must be explicit. Availability must not silently redefine finality.
