// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    approval::{
        ApprovalError, ApprovalRegistry, approval_error_diagnostic, find_approval,
        pin_signed_vote_approval, reconstruct_retained_approval,
    },
    signing::{DurableSigner, sign_vote, signer_status},
};
use anyhow::{Context, Result};
use eve_consensus_comet::wire::tendermint::types::Vote;
use eve_state::StateBudget;

pub(super) fn handle_signer_vote_request(
    signer: &mut DurableSigner,
    registry: &ApprovalRegistry,
    request: Vote,
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<Vote> {
    let before = signer_status(signer).cursor;
    let direct = sign_vote(signer, request.clone(), None);
    let signed = match direct {
        Ok(signed) => signed,
        Err(first) => {
            if first.chain().count() != 1
                || first.to_string() != "non-nil vote requires canonical execution/data approval"
            {
                return Err(first);
            }
            let Some(block) = request
                .block_id
                .as_ref()
                .filter(|block| !block.hash.is_empty())
            else {
                return Err(first);
            };
            let hash: [u8; 32] = block
                .hash
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid requested block hash"))?;
            let approval = match find_approval(registry, &hash).map_err(|error| {
                approval_error_diagnostic(error).context("proposal approval cache failed")
            })? {
                Some(approval) => approval,
                None => match reconstruct_retained_approval(
                    registry,
                    signer,
                    &hash,
                    budget,
                    reserved_clone_bytes,
                ) {
                    Ok(approval) => approval,
                    Err(ApprovalError::Unavailable {
                        reason: "full proposal data unavailable",
                        cause: None,
                    }) => return Err(first),
                    Err(error) => {
                        return Err(approval_error_diagnostic(error)
                            .context("retained proposal execution failed"));
                    }
                },
            };
            sign_vote(signer, request, Some(&approval))
                .context("prepared proposal vote signing failed")?
        }
    };
    if signer_status(signer).cursor != before
        && signed
            .block_id
            .as_ref()
            .is_some_and(|block| !block.hash.is_empty())
    {
        pin_signed_vote_approval(registry, signer, &signed)
            .map_err(|_| anyhow::anyhow!("new signed vote retention pin failed"))?;
    }
    Ok(signed)
}
