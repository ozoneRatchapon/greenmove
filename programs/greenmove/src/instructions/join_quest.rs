use crate::state::{Quest, UserQuestMapping};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct JoinQuest<'info> {
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(init, payer = user, space = 8 + UserQuestMapping::LEN)]
    pub user_quest_mapping: Account<'info, UserQuestMapping>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub quest_pda: Pubkey,
}

pub fn handler(ctx: Context<JoinQuest>, quest_pda: Pubkey) -> Result<()> {
    let user_quest_mapping = &mut ctx.accounts.user_quest_mapping;
    user_quest_mapping.user_pubkey = *ctx.accounts.user.key;
    user_quest_mapping.quest_pda = quest_pda;
    emit!(UserJoinedQuest {
        user_pubkey: ctx.accounts.user.key(),
        quest_pda,
    });
    Ok(())
}

#[event]
pub struct UserJoinedQuest {
    pub user_pubkey: Pubkey,
    pub quest_pda: Pubkey,
}
