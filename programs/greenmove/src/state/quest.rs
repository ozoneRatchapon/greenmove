use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Quest {
    pub quest_pda: Pubkey,
    pub community_leader_pubkey: Pubkey,
    #[max_len(100)]
    pub quest_name: String,
    #[max_len(1000)]
    pub description: String,
    #[max_len(1000)]
    pub conditions: String,
    pub rewards: u64,
    pub deadline: i64,
    #[max_len(50)]
    pub target_audience: Option<String>,
    #[max_len(1000)]
    pub participants: Vec<Pubkey>,
    pub max_participants: u128,
    pub completed: bool,
}
