use anchor_lang::prelude::*;

declare_id!("965niQWV2QJcEJd8WCvmZ5U5CumciAxLLYTYdbhFU8Wi");

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
// use error::GreenmoveError;
// use state::UserAccountState;

#[program]
pub mod greenmove {

    use super::*;

    pub fn create_user(
        ctx: Context<CreateUser>,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        ctx.accounts
            .create_user(
                 display_name, location
                 , ctx.bumps
                )
    }

    pub fn update_user_profile(
        ctx: Context<UpdateUserProfile>,
        // seed: u64,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        ctx.accounts
            .update_user_profile(display_name, location)
    }

    pub fn create_community_leader(
        ctx: Context<CreateCommunityLeader>,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        ctx.accounts
            .create_community_leader(display_name, location, ctx.bumps)
    }

    pub fn log_action(
        ctx: Context<LogAction>,
        action_type: String,
        amount: u64,
        timestamp: i64,
        location: Option<String>,
        proof: Option<String>,
    ) -> Result<()> {
        ctx.accounts
            .log_action(action_type, amount, timestamp, location, proof)
    }

    pub fn get_user_history(ctx: Context<GetUserHistory>) -> Result<()> {
        // TODO:

        let log = &ctx.accounts.action_log_account;
        let user_pubkey = log.user_pubkey.to_string();
        let amount = log.amount;
        let timestamp = log.timestamp;
        let location = log.location.clone();
        let proof = log.proof.clone();
        let action_type = log.action_type.clone();

        msg!("Transaction recorded: user={}, amount={}, action_type={}, timestamp={}, location={}, proof={}", user_pubkey, amount, action_type, timestamp, location.unwrap_or("None".to_string()), proof.unwrap_or("None".to_string()));

        Ok(())
    }

    pub fn create_quest(
        ctx: Context<CreateQuest>,
        quest_name: String,
        description: String,
        conditions: String,
        rewards: u64,
        deadline: i64,
        target_audience: Option<String>,
    ) -> Result<()> {
        msg!("Creating quest: name={}, description={}, conditions={}, rewards={:?}, deadline={}, target_audience={:?}", 
            quest_name, description, conditions, rewards, deadline, target_audience);
        
        // Create the quest
        ctx.accounts.create_quest(
            quest_name.clone(),
            description.clone(),
            conditions.clone(),
            rewards,
            deadline,
            target_audience.clone(),
        )
    }

    // pub fn deposit_rewards(
    //     ctx: Context<CreateQuest>,
    //     reward_amount: u64,
    //     reward_type: String,
    // ) -> Result<()> {
    //     msg!("Depositing rewards: amount={}, type={}", reward_amount, reward_type);
    //     ctx.accounts.deposit_rewards(reward_amount, reward_type)
    // }

    pub fn join_quest(ctx: Context<JoinQuest>, quest_pda: Pubkey) -> Result<()> {
        ctx.accounts.join_quest(quest_pda)
    }

    // pub fn get_quests(ctx: Context<GetQuests>) -> Result<()> {
    //     instructions::get_quests::handler(ctx)
    // }

    // pub fn get_quest_details(ctx: Context<GetQuestDetails>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::get_quest_details::handler(ctx, quest_pda)
    // }

    // pub fn get_user_progress(ctx: Context<GetUserProgress>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::get_user_progress::handler(ctx, quest_pda)
    // }

    // pub fn claim_reward(ctx: Context<ClaimReward>, quest_pda: Pubkey) -> Result<()> {
    //     instructions::claim_reward::handler(ctx, quest_pda)
    // }
}
