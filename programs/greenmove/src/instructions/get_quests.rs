use crate::state::Quest;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetQuests<'info> {
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(mut)]
    pub community_leader: Signer<'info>,
}

pub fn handler(ctx: Context<GetQuests>) -> Result<()> {
    // Logic to fetch all active quests from the blockchain.
    // You can use the quest_account.quest_name to identify the quest.
    let quest_account = &mut ctx.accounts.quest_account;
    quest_account.community_leader_pubkey = *ctx.accounts.community_leader.key;
    quest_account.quest_name = "Sample Quest".to_string();
    quest_account.description = "This is a sample description".to_string();
    quest_account.conditions = vec!["Condition 1".to_string(), "Condition 2".to_string()];
    quest_account.rewards = 123;
    quest_account.deadline = 1673369600; // 2023-01-01 00:00:00 UTC
    quest_account.target_audience = Some("Target Audience".to_string());

    Ok(())
}
