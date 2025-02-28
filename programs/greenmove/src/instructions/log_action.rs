use crate::error::GreenmoveError;
use crate::state::ActionLog;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LogAction<'info> {
    #[account(init, payer = user, space = 8 + ActionLog::LEN)]
    pub action_log_account: Account<'info, ActionLog>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> LogAction<'info> {
    pub fn handler(
        ctx: Context<LogAction>,
        action_type: String,
        amount: u64,
        timestamp: i64,
        location: Option<String>,
        proof: Option<String>,
    ) -> Result<()> {
        if action_type.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        let action_log_account = &mut ctx.accounts.action_log_account;
        action_log_account.user_pubkey = *ctx.accounts.user.key;
        action_log_account.action_type = action_type;
        action_log_account.amount = amount;
        action_log_account.timestamp = timestamp;
        action_log_account.location = location;
        action_log_account.proof = proof;
        emit!(ActionLogged {
            user_pubkey: ctx.accounts.user.key(),
            action_type: action_log_account.action_type.clone(),
            amount: action_log_account.amount,
            timestamp: action_log_account.timestamp,
        });
        Ok(())
    }
}

#[event]
pub struct ActionLogged {
    pub user_pubkey: Pubkey,
    pub action_type: String,
    pub amount: u64,
    pub timestamp: i64,
}
