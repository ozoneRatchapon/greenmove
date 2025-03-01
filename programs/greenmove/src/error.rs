use anchor_lang::prelude::*;

#[error_code]
pub enum GreenmoveError {
    #[msg("Invalid Display Name")]
    InvalidDisplayName,
    #[msg("Invalid Location")]
    InvalidLocation,
    #[msg("Bump Not Found")]
    BumpNotFound,
    #[msg("Invalid Action")]
    InvalidAction,
    #[msg("Invalid Deadline")]
    InvalidDeadline,
    #[msg("Invalid Rewards")]
    InvalidRewards,
    #[msg("Invalid Description")]
    InvalidDescription,
    #[msg("Invalid Conditions")]
    InvalidConditions,



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
