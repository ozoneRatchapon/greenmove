use crate::state::{Quest, UserQuestMapping};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut, has_one = quest_pda)]
    pub quest_account: Account<'info, Quest>,
    #[account(mut, has_one = user_pubkey)]
    pub user_quest_mapping: Account<'info, UserQuestMapping>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimReward<'info> {
    pub fn handler(ctx: Context<ClaimReward>, quest_pda: Pubkey) -> Result<()> {
        let quest_account = &mut ctx.accounts.quest_account;
        let user_quest_mapping = &mut ctx.accounts.user_quest_mapping;
        // Logic to verify that the user has completed the quest conditions, calculate the reward amount, transfer the reward, and update accounts.
        emit!(RewardClaimed {
            user_pubkey: ctx.accounts.user.key(),
            quest_pda,
        });
        Ok(())
    }
}

#[event]
pub struct RewardClaimed {
    pub user_pubkey: Pubkey,
    pub quest_pda: Pubkey,
}
