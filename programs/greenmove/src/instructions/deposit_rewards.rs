use crate::state::{RewardPool, Quest};
use crate::error::GreenmoveError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositRewards<'info> {
    #[account(
        init_if_needed,
        payer = community_leader,
        space = 8 + RewardPool::INIT_SPACE,
        seeds = [b"reward_pool", quest.key().as_ref()],
        bump,
    )]
    pub reward_pool_account: Account<'info, RewardPool>,
    #[account(mut)]
    pub community_leader: Signer<'info>,
    #[account(mut)]
    pub quest: Account<'info, Quest>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositRewards<'info> {
    pub fn deposit_rewards(
        &mut self,
        reward_amount: u64,
        reward_type: String,
    ) -> Result<()> {
        // let reward_pool_account = &mut ctx.accounts.reward_pool_account;
        // Logic to transfer the specified amount of rewards to the reward pool account and update the reward pool balance.
        match reward_type.as_str() {
            "" => return Err(GreenmoveError::InvalidRewardType.into()),
            _ if reward_type.len() > 256 => return Err(GreenmoveError::InvalidRewardType.into()),
            _ => {}
        }

        match reward_amount {
            0 => return Err(GreenmoveError::InvalidAmount.into()),
            _ => {}
        }

        self.reward_pool_account.set_inner(RewardPool {
            quest: self.quest.key(),
            reward_type,
            balance: reward_amount,
        });
        

        Ok(())
    }
}