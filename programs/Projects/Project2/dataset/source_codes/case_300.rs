use anchor_lang::prelude::*;

declare_id!("TeamForm0099999999999999999999999999999999");

#[program]
pub mod team_formation {
    use super::*;

    pub fn set_member(ctx: Context<SetMember>, idx: u8, member: Pubkey) -> Result<()> {
        let team = &mut ctx.accounts.team;
        if (idx as usize) < team.members.len() {
            team.members[idx as usize] = Some(member);
        }
        Ok(())
    }

    pub fn clear_team(ctx: Context<SetMember>) -> Result<()> {
        let team = &mut ctx.accounts.team;
        for slot in team.members.iter_mut() {
            *slot = None;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetMember<'info> {
    #[account(mut)]
    pub team: Account<'info, TeamData>,
}

#[account]
pub struct TeamData {
    pub members: [Option<Pubkey>; 5],
}
