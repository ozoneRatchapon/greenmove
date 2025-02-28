use crate::error::GreenmoveError;
use crate::state::CommunityLeader;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateCommunityLeader<'info> {
    #[account(init, payer = user, space = 8 + CommunityLeader::LEN)]
    pub community_leader_account: Account<'info, CommunityLeader>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCommunityLeader<'info> {
    pub fn handler(
        ctx: Context<CreateCommunityLeader>,
        display_name: String,
        location: Option<String>,
    ) -> Result<()> {
        if display_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        let community_leader_account = &mut ctx.accounts.community_leader_account;
        community_leader_account.user_pubkey = *ctx.accounts.user.key;
        community_leader_account.display_name = display_name;
        community_leader_account.location = location;
        emit!(CommunityLeaderCreated {
            user_pubkey: ctx.accounts.user.key(),
            display_name: community_leader_account.display_name.clone(),
        });
        Ok(())
    }
}

#[event]
pub struct CommunityLeaderCreated {
    pub user_pubkey: Pubkey,
    pub display_name: String,
}
