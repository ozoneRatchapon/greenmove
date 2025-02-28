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
    pub fn create_user(
        &mut self,
        bumps: CreateUserBumps,
        ctx: Context<CreateUser>,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        if display_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        self.user_account_state.user_bump = bumps.user;
        self.user_account_state.state_bump = bumps.user_account_state;
        let user_account = &mut self.user_account_state;
        user_account.user_pubkey = *ctx.accounts.user.key;
        user_account.display_name = display_name;
        user_account.location = location;
        emit!(UserCreated {
            user_pubkey: ctx.accounts.user.key(),
            display_name: user_account.display_name.clone(),
        });
        Ok(())
    }
}

#[event]
pub struct UserCreated {
    pub user_pubkey: Pubkey,
    pub display_name: String,
}
