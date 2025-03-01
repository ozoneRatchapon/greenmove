use anchor_lang::prelude::*;

declare_id!("965niQWV2QJcEJd8WCvmZ5U5CumciAxLLYTYdbhFU8Wi");

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod greenmove {
    use crate::error::GreenmoveError;

    use super::*;

    pub fn create_user(
        ctx: Context<CreateUser>,
        seed: u64,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        ctx.accounts
            .create_user(seed, display_name, location, ctx.bumps)
    }

    // pub fn update_user_profile(
    //     ctx: Context<UpdateUserProfile>,
    //     display_name: String,
    //     location: String,
    // ) -> Result<()> {
    //     instructions::update_user_profile::handler(ctx, display_name, location)
    // }

    // pub fn create_quest(
    //     ctx: Context<CreateQuest>,
    //     quest_name: String,
    //     description: String,
    //     conditions: Vec<String>,
    //     rewards: Vec<String>,
    //     deadline: i64,
    //     target_audience: Option<String>,
    // ) -> Result<()> {
    //     instructions::create_quest::handler(
    //         ctx,
    //         quest_name,
    //         description,
    //         conditions,
    //         rewards,
    //         deadline,
    //         target_audience,
    //     )
    // }

    // pub fn get_quests(ctx: Context<GetQuests>) -> Result<()> {
    //     instructions::get_quests::handler(ctx)
    // }

    // pub fn get_quest_details(ctx: Context<GetQuestDetails>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::get_quest_details::handler(ctx, quest_pda)
    // }

    // pub fn join_quest(ctx: Context<JoinQuest>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::join_quest::handler(ctx, quest_pda)
    // }

    // pub fn log_action(
    //     ctx: Context<LogAction>,
    //     action_type: String,
    //     amount: u64,
    //     timestamp: i64,
    //     location: Option<String>,
    //     proof: Option<String>,
    // ) -> Result<()> {
    //     instructions::log_action::handler(ctx, action_type, amount, timestamp, location, proof)
    // }

    // pub fn get_user_progress(ctx: Context<GetUserProgress>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::get_user_progress::handler(ctx, quest_pda)
    // }

    // pub fn get_user_history(ctx: Context<GetUserHistory>) -> Result<()> {
    //     instructions::get_user_history::handler(ctx)
    // }

    // pub fn claim_reward(ctx: Context<ClaimReward>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::claim_reward::handler(ctx, quest_pda)
    // }

    // pub fn deposit_rewards(
    //     ctx: Context<DepositRewards>,
    //     reward_amount: u64,
    //     reward_type: String,
    // ) -> Result<()> {
    //     instructions::deposit_rewards::handler(ctx, reward_amount, reward_type)
    // }

    // pub fn create_community_leader(
    //     ctx: Context<CreateCommunityLeader>,
    //     display_name: String,
    //     location: Option<String>,
    // ) -> Result<()> {
    //     instructions::create_community_leader::handler(ctx, display_name, location)
    // }
}
