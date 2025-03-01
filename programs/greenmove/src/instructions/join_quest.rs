use crate::error::GreenmoveError;
use crate::state::{Quest, UserQuestMapping};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct JoinQuest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub quest_account: Account<'info, Quest>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserQuestMapping::INIT_SPACE,
        seeds = [b"user_quest_mapping", user.key().as_ref()],
        bump,
    )]
    pub user_quest_mapping: Account<'info, UserQuestMapping>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinQuest<'info> {
    pub fn join_quest(&mut self, quest_pda: Pubkey) -> Result<()> {
        match () {
            _ if self.quest_account.participants.len() >= self.quest_account.max_participants as usize => {
                msg!("Quest is full. Current participants: {}, Max participants: {}", self.quest_account.participants.len(), self.quest_account.max_participants);
                return Err(GreenmoveError::QuestFull.into());
            },
            _ if self.quest_account.participants.contains(&self.user.key()) => {
                return Err(GreenmoveError::UserAlreadyJoined.into());
            },
            _ => (),
        }
        msg!("Joining quest with PDA: {:?}", quest_pda);
        msg!("User pubkey: {:?}", self.user.key());

        self.user_quest_mapping.set_inner(UserQuestMapping {
            user_pubkey: self.user.key(),
            quest_pda,
        });

        msg!("UserQuestMapping set: {:?}", self.user_quest_mapping);

        self.quest_account.participants.push(*self.user.key);
        msg!("Updated quest participants: {:?}", self.quest_account.participants);

        Ok(())
    }
}
