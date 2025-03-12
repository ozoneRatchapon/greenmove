use crate::error::GreenmoveError;
use crate::state::{CommunityLeader, Quest, RewardPool};
// use crate::DepositRewards;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(quest_name: String)]
pub struct CreateQuest<'info> {
    #[account(mut)]
    pub signer_leader: Signer<'info>,
    #[account(
        mut,
        seeds = [b"LeaderState".as_ref(), signer_leader.key().as_ref()],
        bump = community_leader.state_bump,
        constraint = signer_leader.key() == community_leader.user_pubkey @ GreenmoveError::Unauthorized
    )]
    pub community_leader: Account<'info, CommunityLeader>,
    #[account(
        init,
        payer = signer_leader,
        space = 8 + Quest::INIT_SPACE,
        seeds = [
            b"quest".as_ref(),
            signer_leader.key().as_ref(),
            quest_name.as_bytes()
        ],
        bump
    )]
    pub quest_account: Account<'info, Quest>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = signer_leader,
        space = 8 + RewardPool::INIT_SPACE,
        seeds = [b"reward_pool", signer_leader.key().as_ref()],
        bump,
    )]
    pub reward_pool_account: Account<'info, RewardPool>,
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
            }
            _ if description.is_empty() => {
                msg!("Error: description is empty");
                return Err(GreenmoveError::InvalidDescription.into());
            }
            _ if conditions.is_empty() => {
                msg!("Error: conditions are empty");
                return Err(GreenmoveError::InvalidConditions.into());
            }
            _ if rewards == 0 => {
                msg!("Error: rewards is zero");
                return Err(GreenmoveError::InvalidAmount.into());
            }
            _ if deadline < 0 => {
                msg!("Error: deadline is negative");
                return Err(GreenmoveError::InvalidDeadline.into());
            }
            _ if quest_name.len() > 50 => {
                msg!("Error: quest_name is too long");
                return Err(GreenmoveError::InvalidDisplayName.into());
            }
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
        msg!(
            "quest_account set with target_audience: {:?}",
            target_audience
        );

        msg!("Depositing rewards");
        self.deposit_rewards(rewards, "initial_reward".to_string())?;
        msg!("Rewards deposited");

        msg!("Quest created successfully");
        Ok(())
    }

    pub fn deposit_rewards(&mut self, reward_amount: u64, reward_type: String) -> Result<()> {
        match reward_type.as_str() {
            "" => return Err(GreenmoveError::InvalidRewardType.into()),
            _ if reward_type.len() > 256 => return Err(GreenmoveError::InvalidRewardType.into()),
            _ => {}
        }

        match reward_amount {
            0 => return Err(GreenmoveError::InvalidAmount.into()),
            _ => {}
        }

        // Get account info for the signer and reward pool
        let signer_info = self.signer_leader.to_account_info();
        let reward_pool_info = self.reward_pool_account.to_account_info();

        // Calculate the amount needed for the reward pool (reward amount + rent exemption)
        let rent = Rent::get()?;
        let minimum_balance = rent.minimum_balance(reward_pool_info.data_len());

        // Calculate the total amount needed (rewards + minimum balance for rent exemption)
        let total_needed = reward_amount.saturating_add(minimum_balance);

        // Check if signer has enough lamports
        require!(
            signer_info.lamports() >= total_needed,
            GreenmoveError::InsufficientRewards
        );

        // Transfer lamports from signer to reward pool account
        let transfer_amount = reward_amount;
        **signer_info.try_borrow_mut_lamports()? -= transfer_amount;
        **reward_pool_info.try_borrow_mut_lamports()? += transfer_amount;

        // Set the reward pool data
        self.reward_pool_account.set_inner(RewardPool {
            quest: self.quest_account.key(),
            reward_type,
            balance: reward_amount,
        });

        msg!("Transferred {} lamports to reward pool", transfer_amount);
        Ok(())
    }
}
