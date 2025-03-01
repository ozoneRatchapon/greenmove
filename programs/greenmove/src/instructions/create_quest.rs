use crate::error::GreenmoveError;
use crate::state::Quest;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(quest_name: String)]
pub struct CreateQuest<'info> {
    #[account(mut)]
    pub community_leader: Signer<'info>,
    #[account(
        init,
        payer = community_leader,
        space = 8 + Quest::INIT_SPACE,
        seeds = [
            b"quest".as_ref(),
            community_leader.key().as_ref(),
            quest_name.as_bytes()
        ],
        bump
    )]
    pub quest_account: Account<'info, Quest>,
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
        match () {
            _ if quest_name.is_empty() => return Err(GreenmoveError::InvalidDisplayName.into()),
            _ if description.is_empty() => return Err(GreenmoveError::InvalidDescription.into()),
            _ if conditions.is_empty() => return Err(GreenmoveError::InvalidConditions.into()),
            _ if rewards == 0 => return Err(GreenmoveError::InvalidRewards.into()),
            _ if deadline < 0 => return Err(GreenmoveError::InvalidDeadline.into()),
            _ if quest_name.len() > 50 => return Err(GreenmoveError::InvalidDisplayName.into()),
            _ => (),
        }

        let quest_account = &mut self.quest_account;
        quest_account.set_inner(Quest {
            community_leader_pubkey: *self.community_leader.key,
            quest_name,
            description,
            conditions,
            rewards,
            deadline,
            target_audience,
        });
        Ok(())
    }
}
