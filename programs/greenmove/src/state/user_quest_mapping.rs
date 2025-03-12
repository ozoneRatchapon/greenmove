use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserQuestMapping {
    pub user_pubkey: Pubkey,
    pub quest_pda: Pubkey,
    pub is_completed: bool,
    pub is_rewarded: bool,
    pub timestamp: i64,
    pub rewards_claimed: u64,
    pub bump: u8,
}