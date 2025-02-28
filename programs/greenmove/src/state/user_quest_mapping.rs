use anchor_lang::prelude::*;

#[account]
pub struct UserQuestMapping {
    pub user_pubkey: Pubkey,
    pub quest_pda: Pubkey,
}

impl UserQuestMapping {
    pub const LEN: usize = 32 + 32; // Adjust size based on your needs
}
