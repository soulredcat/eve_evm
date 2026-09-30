# Source-build provenance repair — 2026-10-01

The [second hosted attempt](https://github.com/soulredcat/eve_evm/actions/runs/36770126572)
passed pinned Go/Comet archive inspection and extraction, then failed at Comet
build. Its full gate remained skipped. Compact console logging did not expose
the private job's detailed child build log; no hosted pass is inferred.

Local inspection found an independent real provenance defect: the earlier
source-built Comet executable embedded the enclosing EVE revision 4e559fc as its
Go VCS metadata, although the engine source is the pinned upstream 0880b4d commit.
Go's default source build queries the ancestor repository. A real pinned-Go
regression reproduced a failed build when that ancestor Git metadata is invalid.
This explains a concrete possible CI failure path; the unavailable child log
does not prove it was the only cause of the hosted attempt.

The shared source-build command now uses explicit buildvcs=false, readonly
modules, trimpath, the pinned compiler and the upstream engine revision linker
suffix. It never substitutes host repository metadata for source provenance.
Recipe version 2 invalidates prior receipt reuse; source/archive/hash/module,
version and environment checks remain required. Existing receipt/data is not
silently overwritten or accepted under the new recipe.

Actual local checks:

- 88 xtask cases pass: 8 provisioning, 64 structure and 16 verification; no
  failures or ignored cases. The real Go regression observes the original VCS
  failure, uses the production command builder, builds successfully and checks
  absence of an enclosing vcs.revision field.
- Strict xtask all-target Clippy and formatting pass. Structure scan before this
  evidence addition covers 574 files, 15 exclusions and zero violations/warnings.
- The actual upstream Comet source builds with the new recipe; executable SHA256
  5d3c9b3606dd9acebd93bce57196811dad08bf4ad3de373f69d9dda5aeec9dc4.
  Version remains 0.39.0+0880b4d378f347ab16e54ec677ff50d803f37d62.
- A separate clean Linux checkout runs all 13 consensus component tests against
  that executable, including the actual ABCI lifecycle/restart/height fixture.

The whole foundation catalog now has 207 cases. These scoped results are not a
claim that the next hosted/foundation run passed. Observe the actual new run.
Historical d0d4f445 binary results retain their original recipe/host stamp and
must not be presented as this recipe's reproducible artifact.

Raw logs and executable remain local-only under local-tests/consensus-b0-v2 and
the isolated CI checkout; required regression source remains tracked. No source
version, module checksum, security test or authentication requirement is lowered.

The third hosted run confirms full provisioning succeeds, then exposes Git's
runner/container ownership rejection before verification can inspect ignore
rules. Checkout's temporary HOME configuration does not persist into that step.
The job now supplies a command-scoped safe.directory setting for the exact
github.workspace checkout only. No wildcard or user-machine Git setting changes.
The following full hosted result must still be observed separately.
