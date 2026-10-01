// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    approval::{
        ApprovalRegistry, find_approval, pin_signed_vote_approval, reconstruct_retained_approval,
    },
    signing::{DurableSigner, sign_vote, signer_status},
};
use anyhow::Result;
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
            let approval = match find_approval(registry, &hash)
                .map_err(|_| anyhow::anyhow!("approval cache unavailable"))?
            {
                Some(approval) => approval,
                None => reconstruct_retained_approval(
                    registry,
                    signer,
                    &hash,
                    budget,
                    reserved_clone_bytes,
                )
                .map_err(|_| first.context("required proposal execution/data unavailable"))?,
            };
            sign_vote(signer, request, Some(&approval))?
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
