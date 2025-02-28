use crate::error::GreenmoveError;
use crate::state::Quest;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateQuest<'info> {
    #[account(init, payer = community_leader, space = 8 + Quest::LEN)]
    pub quest_account: Account<'info, Quest>,
    #[account(mut)]
    pub community_leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateQuest<'info> {
    pub fn handler(
        ctx: Context<CreateQuest>,
        quest_name: String,
        description: String,
        conditions: Vec<String>,
        rewards: Vec<String>,
        deadline: i64,
        target_audience: Option<String>,
    ) -> Result<()> {
        if quest_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        let quest_account = &mut ctx.accounts.quest_account;
        quest_account.community_leader_pubkey = *ctx.accounts.community_leader.key;
        quest_account.quest_name = quest_name;
        quest_account.description = description;
        quest_account.conditions = conditions;
        quest_account.rewards = rewards;
        quest_account.deadline = deadline;
        quest_account.target_audience = target_audience;
        emit!(QuestCreated {
            community_leader_pubkey: ctx.accounts.community_leader.key(),
            quest_name: quest_account.quest_name.clone(),
        });
        Ok(())
    }
}
#[event]
pub struct QuestCreated {
    pub community_leader_pubkey: Pubkey,
    pub quest_name: String,
}
