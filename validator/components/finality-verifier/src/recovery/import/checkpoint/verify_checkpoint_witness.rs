// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::validate_authenticated_execution_context::validate_authenticated_execution_context;
use super::{
    CheckpointError, CheckpointSession, CheckpointWitness,
    validate_checkpoint_witness_bounds::validate_checkpoint_witness_bounds,
};
use crate::{
    authenticate_current_application_version,
    recovery::verification::verify_recovery_frame::verify_recovery_frame,
};
use eve_state::validate_retained_block;
use std::sync::Arc;

/// Stage one ordered height transactionally; failures never advance the accepted session.
pub fn verify_checkpoint_witness(
    session: &mut CheckpointSession,
    witness: &CheckpointWitness,
    reserved_bytes: usize,
) -> Result<(), CheckpointError> {
    if reserved_bytes < session.required {
        return Err(CheckpointError::ReservationTooSmall);
    }
    if session.closing_data.is_some() {
        return Err(CheckpointError::WrongHeight);
    }
    validate_checkpoint_witness_bounds(session, witness)?;
    let native = match witness {
        CheckpointWitness::Execution(execution) => &execution.native,
        CheckpointWitness::Lookahead(native) => native.as_ref(),
    };
    if u64::try_from(native.frame.header.height).ok() != Some(session.next_height) {
        return Err(CheckpointError::WrongHeight);
    }
    let mut finality = session.finality.clone();
    let certified = if session.next_height == session.base_height + 1 && session.base_height > 0 {
        let seed = session
            .seed_data
            .as_ref()
            .ok_or(CheckpointError::WrongParent)?;
        if seed.as_ref() != native {
            return Err(CheckpointError::WrongNativeData);
        }
        session
            .seed_header
            .as_ref()
            .ok_or(CheckpointError::WrongParent)?
            .clone()
    } else {
        verify_recovery_frame(
            &mut finality,
            &native.frame,
            &native.transactions,
            &session.policy,
        )
        .map_err(CheckpointError::Recovery)?
    };
    if session.previous_version.height > 0 {
        authenticate_current_application_version(&finality, &session.previous_version)
            .map_err(|error| CheckpointError::Recovery(crate::RecoveryError::Finality(error)))?;
    }
    let next = session
        .next_height
        .checked_add(1)
        .ok_or(CheckpointError::Overflow)?;
    match witness {
        CheckpointWitness::Execution(execution) => {
            let version = &execution.version;
            let block = &execution.block;
            if session.next_height > session.target.target.height
                || version.height != session.next_height
                || version.identity != session.target.target.identity
            {
                return Err(CheckpointError::WrongHeight);
            }
            if block.transactions != native.transactions {
                return Err(CheckpointError::WrongNativeData);
            }
            validate_retained_block(
                version,
                Some(&session.previous_version),
                block,
                &session.budget,
            )
            .map_err(CheckpointError::State)?;
            validate_authenticated_execution_context(
                &session.previous_version,
                &session.previous_header,
                &block.header,
                &certified,
                &session.policy,
            )
            .map_err(CheckpointError::Import)?;
            if session.target.state.block_hashes.get(&version.height)
                != Some(&version.execution_hash)
            {
                return Err(CheckpointError::WrongHistory);
            }
            if version.height == session.target.target.height
                && (version != &session.target.target
                    || block != &session.target.block
                    || session.target.parent.as_ref() != Some(&session.previous_version))
            {
                return Err(CheckpointError::WrongTarget);
            }
            session.previous_version = version.clone();
            session.previous_header = block.header.clone();
        }
        CheckpointWitness::Lookahead(_) => {
            if session.next_height != session.target.target.height + 1
                || session.previous_version != session.target.target
            {
                return Err(CheckpointError::WrongHeight);
            }
            session.closing_data = Some(Arc::new(native.clone()));
            session.closing_header = Some(certified);
        }
    }
    session.finality = finality;
    session.next_height = next;
    Ok(())
}
