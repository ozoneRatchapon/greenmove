use anchor_lang::prelude::*;

#[account]
pub struct Quest {
    // pub quest_pda: Pubkey,
    pub community_leader_pubkey: Pubkey,
    pub quest_name: String,
    pub description: String,
    pub conditions: Vec<String>,
    pub rewards: u64,
    pub deadline: i64,
    pub country: String,
    pub target_audience: Option<String>,
}

// impl Quest {
//     pub const LEN: usize = 32 + 4 + 32 + 4 + 256 + 4 * 10 + 4 * 10 + 8 + 4 * 10; // Adjust size based on your needs
// }
