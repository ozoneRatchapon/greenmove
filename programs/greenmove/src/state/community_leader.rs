use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CommunityLeader {
    pub seed: u64, 
    pub user_bump: u8,
    pub state_bump: u8,
    pub user_pubkey: Pubkey,
    #[max_len(50)]
    pub display_name: String,
    #[max_len(50)]
    pub location: Option<String>,
}
