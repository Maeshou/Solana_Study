
use anchor_lang::prelude::*;

declare_id!("Captain1111111111111111111111111111111111111");

#[program]
pub mod case1 {
    use super::*;

    pub fn change_captain(ctx: Context<ChangeCaptain>, remarks: String) -> Result<()> {
        let team = &mut ctx.accounts.team;
        let old_captain = team.captain;
        team.captain = ctx.accounts.new_captain.key();
        team.history.push(remarks);
        msg!("Captain changed from {:?} to {:?}", old_captain, team.captain);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ChangeCaptain<'info> {
    #[account(mut)]
    pub team: Account<'info, Team>,
    pub new_captain: Signer<'info>,
    /// CHECK: missing signer + no ownership validation
    pub requester: UncheckedAccount<'info>,
}

#[account]
pub struct Team {
    pub captain: Pubkey,
    pub manager: Pubkey,
    pub history: Vec<String>,
}
