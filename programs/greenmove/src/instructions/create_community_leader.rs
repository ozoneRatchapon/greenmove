use crate::error::GreenmoveError;
use crate::state::CommunityLeader;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateCommunityLeader<'info> {
    #[account(mut)]
    pub signer_leader: Signer<'info>,
    #[account(
        init_if_needed,
        payer = signer_leader,
        space = 8 + CommunityLeader::INIT_SPACE,
        seeds = [b"LeaderState", signer_leader.key().as_ref()],
        bump,
    )]
    pub community_leader_account: Account<'info, CommunityLeader>,
    #[account(seeds = [community_leader_account.key().as_ref()], bump)]
    pub community_leader: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCommunityLeader<'info> {
    pub fn create_community_leader(
        &mut self,
        seed: u64,
        display_name: String,
        location: Option<String>,
        bumps: CreateCommunityLeaderBumps,
    ) -> Result<()> {
        if display_name.is_empty() {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        if display_name.len() > 100 {
            return Err(GreenmoveError::InvalidDisplayName.into());
        }
        if let Some(ref loc) = location
        {
            if loc.len() > 100 {
                return Err(GreenmoveError::InvalidLocation.into());
            }
        }

        self.community_leader_account.set_inner(CommunityLeader {
            seed,
            user_bump: bumps.community_leader,
            state_bump: bumps.community_leader_account,
            user_pubkey: self.community_leader.key(),
            display_name,
            location,
        });
        
        Ok(())
    }
}
