use crate::error::GreenmoveError;
use crate::state::UserAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateUserProfile<'info> {
    #[account(mut, has_one = user_pubkey)]
    pub user_account: Account<'info, UserAccount>,
    pub user_pubkey: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateUserProfile>,
    display_name: String,
    location: String,
) -> Result<()> {
    if display_name.is_empty() {
        return Err(GreenmoveError::InvalidDisplayName.into());
    }
    let user_account = &mut ctx.accounts.user_account;
    user_account.display_name = display_name;
    user_account.location = Some(location);
    emit!(UserProfileUpdated {
        user_pubkey: *ctx.accounts.user_pubkey.key,
        display_name: user_account.display_name.clone(),
        location: user_account.location.clone(),
    });
    Ok(())
}

#[event]
pub struct UserProfileUpdated {
    pub user_pubkey: Pubkey,
    pub display_name: String,
    pub location: Option<String>,
}
