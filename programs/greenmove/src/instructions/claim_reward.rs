use crate::error::GreenmoveError;
use crate::state::{Quest, RewardPool, UserQuestMapping};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(mut,
        seeds = [
            b"user_quest_mapping".as_ref(),
            user.key().as_ref(),
            quest_account.key().as_ref(),
        ],
        bump = user_quest_mapping.bump,
        constraint = user.key() == user_quest_mapping.user_pubkey @ GreenmoveError::Unauthorized
    )]
    pub user_quest_mapping: Account<'info, UserQuestMapping>,
    #[account(mut)]
    pub reward_pool_account: Account<'info, RewardPool>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
    pub fn claim_reward(&mut self, _quest_pda: Pubkey) -> Result<()> {
        let quest_account = &mut self.quest_account;
        let user_quest_mapping = &mut self.user_quest_mapping;
        let user = &mut self.user;
        let reward_pool = &mut self.reward_pool_account;

        // Verify that both the quest and user's participation are completed
        require!(quest_account.completed, GreenmoveError::QuestNotCompleted);
        require!(
            user_quest_mapping.is_completed,
            GreenmoveError::QuestNotCompleted
        );
        require!(
            !user_quest_mapping.is_rewarded,
            GreenmoveError::RewardAlreadyClaimed
        );

        // Calculate the reward amount
        let reward_amount = quest_account.rewards;

        // Verify reward pool has enough balance
        require!(
            reward_pool.balance >= reward_amount,
            GreenmoveError::InsufficientRewards
        );

        // Calculate minimum rent exemption for the reward pool account
        let reward_pool_info = reward_pool.to_account_info();
        let user_info = user.to_account_info();

        // Get the minimum rent exemption amount for the reward pool account
        let rent = Rent::get()?;
        let minimum_balance = rent.minimum_balance(reward_pool_info.data_len());

        // Calculate the maximum amount that can be transferred while maintaining rent exemption
        let current_lamports = reward_pool_info.lamports();
        let available_for_transfer = current_lamports.saturating_sub(minimum_balance);

        // Ensure we don't transfer more than what's available after rent exemption
        let actual_transfer_amount = reward_amount.min(available_for_transfer);
        // If there are no funds available for transfer, log a message and return an error
        if actual_transfer_amount == 0 {
            msg!("No funds available for transfer after maintaining rent exemption");
            return Err(GreenmoveError::InsufficientRewards.into());
        }

        // Transfer the reward to the user from the reward pool account
        **reward_pool_info.try_borrow_mut_lamports()? -= actual_transfer_amount;
        **user_info.try_borrow_mut_lamports()? += actual_transfer_amount;

        // Update the reward pool balance
        reward_pool.balance -= actual_transfer_amount;

        // Update the user_quest_mapping to reflect that the reward has been claimed
        user_quest_mapping.is_rewarded = true;
        user_quest_mapping.rewards_claimed = actual_transfer_amount;

        // Update the quest rewards to reflect they've been claimed
        quest_account.rewards = 0;

        Ok(())
    }
}
