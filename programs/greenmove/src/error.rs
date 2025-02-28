use anchor_lang::prelude::*;
use thiserror::Error;

#[error_code]
pub enum GreenmoveError {
    #[msg("Invalid Display Name")]
    InvalidDisplayName,
    #[msg("Invalid Location")]
    InvalidLocation,
    #[msg("Quest Not Found")]
    QuestNotFound,
    #[msg("User Not Found")]
    UserNotFound,
    #[msg("Action Not Allowed")]
    ActionNotAllowed,
    #[msg("Insufficient Rewards")]
    InsufficientRewards,
    #[msg("Invalid Proof")]
    InvalidProof,
    #[msg("Quest Already Joined")]
    QuestAlreadyJoined,
    #[msg("Quest Not Completed")]
    QuestNotCompleted,
    #[msg("Unauthorized")]
    Unauthorized,
}

impl From<GreenmoveError> for ProgramError {
    fn from(e: GreenmoveError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
