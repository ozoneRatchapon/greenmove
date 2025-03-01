use crate::error::GreenmoveError;
use crate::state::ActionLog;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct LogAction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + ActionLog::INIT_SPACE,
        seeds = [b"action_log", user.key().as_ref()],
        bump,
    )]
    pub action_log_account: Account<'info, ActionLog>,

    pub system_program: Program<'info, System>,
}

impl<'info> LogAction<'info> {
    pub fn log_action(
        &mut self,
        action_type: String,
        amount: u64,
        timestamp: i64,
        location: Option<String>,
        proof: Option<String>,
    ) -> Result<()> {
        match action_type.as_str() {
            "" => return Err(GreenmoveError::InvalidAction.into()),
            _ if action_type.len() > 256 => return Err(GreenmoveError::InvalidAction.into()),
            _ => {}
        }

        match location {
            Some(ref loc) if loc.len() > 50 => return Err(GreenmoveError::InvalidLocation.into()),
            _ => {}
        }

        match proof {
            Some(ref prf) if prf.len() > 256 => return Err(GreenmoveError::InvalidProof.into()),
            _ => {}
        }

        match amount {
            0 => return Err(GreenmoveError::InvalidAmount.into()),
            _ => {}
        }

        self.action_log_account.set_inner(ActionLog {
            user_pubkey: self.user.key(),
            action_type,
            amount,
            timestamp,
            location,
            proof,
        });
        Ok(())
    }
}