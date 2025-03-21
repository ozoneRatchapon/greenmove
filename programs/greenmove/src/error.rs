use anchor_lang::prelude::*;

#[error_code]
// #[derive(AnchorDeserialize, AnchorSerialize)]
pub enum GreenmoveError {
    #[msg("Quest not completed")]
    QuestNotCompleted,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Reward already claimed")]
    RewardAlreadyClaimed,
    #[msg("Account not found")]
    AccountNotFound,
    #[msg("Invalid Display Name")]
    InvalidDisplayName,
    #[msg("Invalid Location")]
    InvalidLocation,
    #[msg("Invalid Reward Pool")]
    InvalidRewardPool,
    #[msg("Invalid Reward Pool Balance")]
    InvalidRewardPoolBalance,
    #[msg("Invalid Reward Pool Type")]
    InvalidRewardPoolType,
    #[msg("Invalid Reward Pool Quest")]
    InvalidRewardPoolQuest,
    #[msg("Invalid Reward Pool Account")]
    InvalidRewardPoolAccount,
    #[msg("Display Name Too Long")]
    DisplayNameTooLong,
    #[msg("Display Name Empty")]
    DisplayNameEmpty,
    #[msg("Location Too Long")]
    LocationTooLong,
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
    #[msg("User not joined quest")]
    UserNotJoined,
}

impl From<GreenmoveError> for ProgramError {
    fn from(e: GreenmoveError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
