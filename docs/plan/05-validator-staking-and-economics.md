# 05 — Validator, Staking and Economics

## Initial fee proposal

For total protocol transaction fees per accounting period:

```text
40% → burn
30% → node reward pool
30% → validator reward pool
```

Use integer basis points to avoid floating-point consensus math:

```text
burn_bps      = 4000
node_bps      = 3000
validator_bps = 3000
total         = 10000
```

These values are proposed parameters, not yet frozen protocol constants.

## Reward timing

Calculate/distribute rewards per epoch rather than adding expensive accounting to every EVM transaction hot path.

## Validator eligibility

Potential components:

- minimum self stake;
- delegated stake;
- active validator-set membership;
- uptime/liveness;
- timely consensus participation;
- valid proposal history;
- correctness;
- slash/jail state.

## Node reward eligibility

A node must be rewarded only for useful protocol-verifiable work.

Candidate work classes:

- data availability/storage challenge responses;
- execution tasks;
- state replication;
- archival duties;
- relaying only if contribution can be measured without trivial Sybil abuse.

Pure self-reported "work completed" is not acceptable.

## Scoring

An eventual scoring function may combine:

- availability;
- verified work;
- stake weight where appropriate;
- correctness;
- participation.

All scoring inputs must be deterministic and auditable.

## Slashing / penalties

Define explicitly before mainnet:

- double signing;
- invalid vote/proposal;
- prolonged unavailability;
- provably incorrect execution result;
- equivocation;
- conditions that cause jailing vs stake loss.

Delegator exposure to validator slashing must also be specified.

## Open questions

- staking asset;
- inflation vs fee-only rewards;
- minimum validator stake;
- delegation cap;
- unbonding period;
- commission limits;
- governance of fee parameters;
- whether node rewards require stake.
