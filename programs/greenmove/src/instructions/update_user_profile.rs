use crate::error::GreenmoveError;
use crate::state::UserAccountState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateUserProfile<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,
        seeds = [b"state", signer.key().as_ref()],
        bump
    )]
    pub user_account_state: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateUserProfile<'info> {
    pub fn update_user_profile(
        &mut self,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        if display_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        self.user_account_state.set_inner(UserAccountState {
            seed: self.user_account_state.seed,
            state_bump: self.user_account_state.state_bump,
            user_pubkey: self.signer.key(),
            display_name,
            location,
        });
        Ok(())
    }
}
