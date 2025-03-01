use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserQuestMapping {
    pub user_pubkey: Pubkey,
    pub quest_pda: Pubkey,
}