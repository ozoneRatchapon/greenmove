use anchor_lang::prelude::*;

#[account]
pub struct ActionLog {
    pub user_pubkey: Pubkey,
    pub action_type: String,
    pub amount: u64,
    pub timestamp: i64,
    // pub location: Option<String>,
    // pub proof: Option<String>,
}

// impl ActionLog {
//     pub const LEN: usize = 32 + 4 + 32 + 8 + 8 + 4 * 10 + 4 * 10; // Adjust size based on your needs
// }
