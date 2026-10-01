<!-- SPDX-FileCopyrightText: 2026 Redcat -->
<!-- SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only -->
<!-- Use requires prior written permission from Redcat. -->

# Primary implementation references

Reviewed for planning on 2026-09-30. These links explain upstream behavior; they are not dependency/version pins or evidence that EVE has been implemented. B0 must record actual selected revisions, digests and compatibility tests.

## Coding-agent execution

- [OpenAI: AGENTS.md guidance](https://developers.openai.com/codex/guides/agents-md): repository instructions and scoped discovery.
- [OpenAI: Codex slash commands](https://developers.openai.com/codex/cli/slash-commands): `/goal <objective>` and goal lifecycle. Longer instructions belong in a file referenced by the goal; the documented objective limit is 4,000 characters.
- [OpenAI: long-horizon workflows](https://developers.openai.com/blog/run-long-horizon-tasks-with-codex): persistent plans, validation and execution documentation.
- [OpenAI: execution plans](https://developers.openai.com/cookbook/articles/codex_exec_plans): self-contained, resumable implementation plans.

The project's role files and bulk policy are custom instructions. Their existence does not launch subagents, bypass permissions or guarantee unlimited execution.

## Consensus and authentication

- [CometBFT introduction](https://docs.cosmos.network/cometbft/latest/docs/introduction/intro): BFT application replication and the separation of engine/application.
- [ABCI methods](https://docs.cosmos.network/cometbft/latest/spec/abci/Methods): actual request/response lifecycle, next-height application-hash commitment and validator-update timing.
- [ABCI application requirements](https://docs.cosmos.network/cometbft/latest/spec/abci/Requirements-for-the-Application): deterministic behavior and side-effect constraints.
- [Light-client verification](https://docs.cosmos.network/cometbft/latest/spec/light-client/verification): authenticated history/checkpoint verification.

EVE's commitment mapping, role separation and development economics are project design choices built around these constraints. Do not treat a bare quorum-signature count as a complete implementation of the upstream consensus protocol.

## EVM, encoding and storage

- [REVM source](https://github.com/bluealloy/revm) and [REVM API](https://docs.rs/revm): Rust execution engine and supported integration interfaces.
- [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559): type-2 transaction/pricing rules. EVE deliberately differs in fee redistribution.
- [Ethereum Merkle Patricia trie](https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/): account/storage and ordered transaction/receipt commitments.
- [Ethereum JSON-RPC reference](https://ethereum.org/en/developers/docs/apis/json-rpc/): method/encoding conventions; EVE finality and compatibility differences remain explicit.
- [RocksDB WAL](https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-%28WAL%29): log/durability reference. Actual sync settings and crash guarantees require tests.
- [Execution-test migration notice](https://eest.ethereum.org/): the former EEST documentation reports that tests merged into `ethereum/execution-specs`. Follow its maintained repository/documentation links and pin the selected test corpus; do not rely on an archived path simply because older instructions name it.

## Interpretation rule

External references do not override user decisions or EVE specifications silently. When upstream behavior conflicts with a draft assumption, record the conflict, test it, update the adapter/spec/tests together and preserve the security boundary. Published benchmark numbers from another project never count as EVE's capacity evidence.
