use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardPool {
    pub quest: Pubkey,
    #[max_len(256)]
    pub reward_type: String,
    pub balance: u64,
}