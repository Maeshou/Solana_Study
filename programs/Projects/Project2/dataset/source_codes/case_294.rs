use anchor_lang::prelude::*;

declare_id!("LandGrid0033333333333333333333333333333333");

#[program]
pub mod land_claim {
    use super::*;

    pub fn claim(ctx: Context<ClaimLand>, x: u8, y: u8) -> Result<()> {
        let grid = &mut ctx.accounts.grid;
        if (x as usize) < 10 && (y as usize) < 10 {
            grid.cells[(y as usize)][(x as usize)] = Some(ctx.accounts.user.key());
            grid.claims = grid.claims.saturating_add(1);
        }
        Ok(())
    }

    pub fn reset(ctx: Context<ClaimLand>) -> Result<()> {
        let grid = &mut ctx.accounts.grid;
        for row in grid.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = None;
            }
        }
        grid.claims = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimLand<'info> {
    #[account(mut)]
    pub grid: Account<'info, LandGrid>,
    pub user: Signer<'info>,
}

#[account]
pub struct LandGrid {
    pub cells: [[Option<Pubkey>; 10]; 10],
    pub claims: u64,
}
