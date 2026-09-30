use core::marker::PhantomData;
use revm::{
    context_interface::{ContextTr, JournalTr, result::HaltReason},
    handler::{EvmTr, EvmTrError, FrameResult, FrameTr, Handler},
    interpreter::interpreter_action::FrameInit,
    state::EvmState,
};

/// Keep default validation/deduction/refunds while suppressing Ethereum tips.
pub(crate) struct EveFeeHandler<EVM, ERROR, FRAME> {
    pub marker: PhantomData<(EVM, ERROR, FRAME)>,
}

impl<EVM, ERROR, FRAME> Handler for EveFeeHandler<EVM, ERROR, FRAME>
where
    EVM: EvmTr<Context: ContextTr<Journal: JournalTr<State = EvmState>>, Frame = FRAME>,
    ERROR: EvmTrError<EVM>,
    FRAME: FrameTr<FrameResult = FrameResult, FrameInit = FrameInit>,
{
    type Evm = EVM;
    type Error = ERROR;
    type HaltReason = HaltReason;

    fn reward_beneficiary(
        &self,
        _evm: &mut Self::Evm,
        _execution: &mut FrameResult,
    ) -> Result<(), Self::Error> {
        super::suppress_proposer_reward::suppress_proposer_reward()
    }
}
