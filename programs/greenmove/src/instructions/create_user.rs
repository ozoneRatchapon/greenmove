use crate::error::GreenmoveError;
use crate::state::UserAccountState;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + UserAccountState::INIT_SPACE,
        seeds = [b"state", signer.key().as_ref()],
        bump,
    )]
    pub user_account_state: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateUser<'info> {
    /// Creates a new user account.
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed used to derive the account.
    /// * `bumps` - Bump seeds for the accounts.
    /// * `ctx` - The context of the transaction.
    /// * `display_name` - The display name of the user.
    /// * `location` - The optional location of the user.
    pub fn create_user(
        &mut self,
        // seed: u64,
        display_name: String,
        location: Option<String>,
        bumps: CreateUserBumps,
    ) -> Result<()> {
        match display_name.len() {
            0 => return Err(GreenmoveError::DisplayNameEmpty.into()),
            len if len > 100 => return Err(GreenmoveError::DisplayNameTooLong.into()),
            _ => {}
        }

        if let Some(ref loc) = location {
            if loc.len() > 100 {
            return Err(GreenmoveError::LocationTooLong.into());
            }
        }

        self.user_account_state.set_inner(UserAccountState {
            // seed,
            state_bump: bumps.user_account_state,
            user_pubkey: self.signer.key(),
            display_name,
            location,
        });
        Ok(())
    }
}
