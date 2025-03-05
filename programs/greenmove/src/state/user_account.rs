use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState {
    pub state_bump: u8,
    pub user_pubkey: Pubkey, // maker
    #[max_len(50)]
    pub display_name: String,
    #[max_len(50)]
    pub location: Option<String>,
}