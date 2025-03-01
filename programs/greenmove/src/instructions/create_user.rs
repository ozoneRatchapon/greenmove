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
    #[account(seeds = [user_account_state.key().as_ref()], bump)]
    pub user: SystemAccount<'info>,
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
        seed: u64,
        display_name: String,
        location: Option<String>,
        bumps: CreateUserBumps,
    ) -> Result<()> {
        // Validate display name
        if display_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        if display_name.len() > 100 {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }

        // Validate location
        if let Some(ref loc) = location {
            if loc.len() > 100 {
                return Err(GreenmoveError::InvalidLocation.into());
            }
        }

        self.user_account_state.set_inner(UserAccountState {
            seed,
            user_bump: bumps.user,
            state_bump: bumps.user_account_state,
            user_pubkey: self.user.key(),
            display_name,
            location,
        });
        Ok(())
    }
}
