use crate::state::RewardPool;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositRewards<'info> {
    #[account(mut)]
    pub reward_pool_account: Account<'info, RewardPool>,
    #[account(mut)]
    pub community_leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositRewards<'info> {
    pub fn handler(
        ctx: Context<DepositRewards>,
        reward_amount: u64,
        reward_type: String,
    ) -> Result<()> {
        let reward_pool_account = &mut ctx.accounts.reward_pool_account;
        // Logic to transfer the specified amount of rewards to the reward pool account and update the reward pool balance.
        emit!(RewardsDeposited {
            community_leader_pubkey: ctx.accounts.community_leader.key(),
            reward_amount,
            reward_type,
        });
        Ok(())
    }
}

#[event]
pub struct RewardsDeposited {
    pub community_leader_pubkey: Pubkey,
    pub reward_amount: u64,
    pub reward_type: String,
}
