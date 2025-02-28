use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState {
    pub seed: u64, // seeds used to derive the account
    pub user_bump: u8,
    pub state_bump: u8,
    pub user_pubkey: Pubkey, // maker
    #[max_len(100)]
    pub display_name: String,
    #[max_len(100)]
    pub location: Option<String>,
}

// impl UserAccount {
//     pub const LEN: usize = 32 + 4 + 32 + 4 + 256; // Adjust size based on your needs
// }
