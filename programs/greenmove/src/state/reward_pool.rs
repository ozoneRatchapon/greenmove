use anchor_lang::prelude::*;

#[account]
pub struct RewardPool {
    pub quest: Pubkey,
    pub balance: u64,
}

// impl RewardPool {
//     pub const LEN: usize = 32 + 8 + 4 * 10; // Adjust size based on your needs
// }
