use anchor_lang::prelude::*;

#[account]
pub struct CommunityLeader {
    pub user_pubkey: Pubkey,
    pub display_name: String,
    pub location: Option<String>,
}

impl CommunityLeader {
    pub const LEN: usize = 32 + 4 + 32 + 4 + 256; // Adjust size based on your needs
}
