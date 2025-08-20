use anchor_lang::prelude::*;

declare_id!("Treasure05555555555555555555555555555555");

#[program]
pub mod treasure_hunt {
    use super::*;

    pub fn discover(ctx: Context<Discover>, index: u8) -> Result<()> {
        let th = &mut ctx.accounts.hunt;
        if (index as usize) < th.found.len() {
            th.found[index as usize] = true;
            th.total_found = th.total_found.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Discover<'info> {
    #[account(mut)]
    pub hunt: Account<'info, HuntData>,
}

#[account]
pub struct HuntData {
    pub found: [bool; 16],
    pub total_found: u8,
}
