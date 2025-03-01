use crate::error::GreenmoveError;
use crate::state::{Quest, CommunityLeader};
use anchor_lang::prelude::*;
#[derive(Accounts)]
#[instruction(quest_name: String)]
pub struct CreateQuest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"community_leader".as_ref()],
        bump = community_leader.user_bump,
        constraint = signer.key() == community_leader.key() @ GreenmoveError::Unauthorized
    )]
    pub community_leader: Account<'info, CommunityLeader>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + Quest::INIT_SPACE,
        seeds = [
            b"quest".as_ref(),
            community_leader.key().as_ref(),
            quest_name.as_bytes()
        ],
        bump
    )]
    pub quest_account: Account<'info, Quest>,
    #[account(seeds = [quest_account.key().as_ref()], bump)]
    pub quest_pda: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateQuest<'info> {
    pub fn create_quest(
        &mut self,
        quest_name: String,
        description: String,
        conditions: String,
        rewards: u64,
        deadline: i64,
        target_audience: Option<String>,
    ) -> Result<()> {
        msg!("Starting create_quest function");
        msg!("quest_name: {}", quest_name);
        msg!("description: {}", description);
        msg!("conditions: {}", conditions);
        msg!("rewards: {}", rewards);
        msg!("deadline: {}", deadline);
        msg!("target_audience: {:?}", target_audience);

        match () {
            _ if quest_name.is_empty() => {
                msg!("Error: quest_name is empty");
                return Err(GreenmoveError::InvalidDisplayName.into());
            },
            _ if description.is_empty() => {
                msg!("Error: description is empty");
                return Err(GreenmoveError::InvalidDescription.into());
            },
            _ if conditions.is_empty() => {
                msg!("Error: conditions are empty");
                return Err(GreenmoveError::InvalidConditions.into());
            },
            _ if rewards == 0 => {
                msg!("Error: rewards is zero");
                return Err(GreenmoveError::InvalidAmount.into());
            },
            _ if deadline < 0 => {
                msg!("Error: deadline is negative");
                return Err(GreenmoveError::InvalidDeadline.into());
            },
            _ if quest_name.len() > 50 => {
                msg!("Error: quest_name is too long");
                return Err(GreenmoveError::InvalidDisplayName.into());
            },
            _ => (),
        }

        let quest_account = &mut self.quest_account;
        msg!("Setting quest_account inner values");
        quest_account.set_inner(Quest {
            quest_pda: quest_account.key(),
            community_leader_pubkey: self.community_leader.key(),
            quest_name: quest_name.clone(),
            description: description.clone(),
            conditions: conditions.clone(),
            rewards,
            deadline,
            target_audience: target_audience.clone(),
            participants: vec![],
            max_participants: 10000,
            completed: false,
        });
        msg!("quest_account set with quest_name: {}", quest_name);
        msg!("quest_account set with description: {}", description);
        msg!("quest_account set with conditions: {}", conditions);
        msg!("quest_account set with rewards: {}", rewards);
        msg!("quest_account set with deadline: {}", deadline);
        msg!("quest_account set with target_audience: {:?}", target_audience);

        msg!("Quest created successfully");
        Ok(())
    }
}
