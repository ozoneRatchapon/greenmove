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
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Invalid Rewards Type")]
    InvalidRewardType,
    #[msg("Invalid Description")]
    InvalidDescription,
    #[msg("Invalid Conditions")]
    InvalidConditions,
    #[msg("Invalid Quest Name")]
    InvalidQuestName,
    #[msg("Invalid Quest Description")]
    InvalidQuestDescription,
    #[msg("Invalid Quest Conditions")]
    InvalidQuestConditions,
    #[msg("Invalid Quest Rewards")]
    InvalidQuestRewards,
    #[msg("Invalid Quest Deadline")]
    InvalidQuestDeadline,
    #[msg("Invalid Quest Target Audience")]
    InvalidQuestTargetAudience,
    #[msg("User Already Joined")]
    UserAlreadyJoined,
    #[msg("Quest Full")]
    QuestFull,
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
