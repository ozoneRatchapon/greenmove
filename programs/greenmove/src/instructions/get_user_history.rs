use crate::state::ActionLog;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetUserHistory<'info> {
    #[account(has_one = user_pubkey)]
    pub action_log_account: Account<'info, ActionLog>,
    pub user_pubkey: Signer<'info>,
}
impl<'info> GetUserHistory<'info> {
    pub fn get_user_history(
        &mut self,
        ctx: &Context<GetUserHistory>
    ) -> Result<()> {
        let log = &ctx.accounts.action_log_account;
        let user_pubkey = log.user_pubkey.to_string();
        let amount = log.amount;
        let timestamp = log.timestamp;
        let location = log.location.clone();
        let proof = log.proof.clone();
        let action_type = log.action_type.clone();
    
        msg!("Transaction recorded: user={}, amount={}, action_type={}, timestamp={}, location={}, proof={}", user_pubkey, amount, log.action_type, timestamp, location.unwrap_or("None".to_string()), proof.unwrap_or("None".to_string()));
    
        Ok(())
    }
}
