use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ActionLog {
    pub user_pubkey: Pubkey,
    #[max_len(256)]
    pub action_type: String,
    pub amount: u64,
    pub timestamp: i64,
    #[max_len(50)]
    pub location: Option<String>,
    #[max_len(256)]
    pub proof: Option<String>,
}
