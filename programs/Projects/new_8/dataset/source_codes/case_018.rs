use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("FaRmHarVeSt0000000000000000000000000006");

#[program]
pub mod farm_harvest {
    use super::*;

    pub fn plant_seed(ctx: Context<PlantSeed>, field: [u8; 8], water: u16, bump: u8) -> Result<()> {
        let mut f = field;
        for i in 0..f.len() {
            if !f[i].is_ascii_alphanumeric() { f[i] = b'1' + (i as u8 % 9); }
        }
        let mut w = water as u32;
        if w > 4000 { w = 4000; }

        let seeds = [&ctx.accounts.farmer.key().to_bytes()[..], &f[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(FarmErr::Cell))?;
        if addr != ctx.accounts.plot_cell.key() { return Err(error!(FarmErr::Cell)); }

        let p = &mut ctx.accounts.plot;
        p.farmer = ctx.accounts.farmer.key();
        p.field = f;
        p.moist = p.moist.saturating_add(w);
        p.cycle = p.cycle.wrapping_add(1);
        Ok(())
    }

    pub fn collect_crop(ctx: Context<CollectCrop>, field: [u8; 8], bump: u8) -> Result<()> {
        let seeds = [&ctx.accounts.farmer.key().to_bytes()[..], &field[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(FarmErr::Cell))?;
        if addr != ctx.accounts.plot_cell.key() { return Err(error!(FarmErr::Cell)); }

        let p = &mut ctx.accounts.plot;
        let gained = (p.moist / 9) as u64;
        p.harvest = p.harvest.saturating_add(gained);
        p.moist = p.moist.saturating_sub((gained as u32) * 8);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PlantSeed<'info> {
    #[account(mut)]
    pub plot: Account<'info, Plot>,
    /// CHECK:
    pub plot_cell: AccountInfo<'info>,
    pub farmer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectCrop<'info> {
    #[account(mut)]
    pub plot: Account<'info, Plot>,
    /// CHECK:
    pub plot_cell: AccountInfo<'info>,
    pub farmer: AccountInfo<'info>,
}

#[account]
pub struct Plot {
    pub farmer: Pubkey,
    pub field: [u8; 8],
    pub moist: u32,
    pub harvest: u64,
    pub cycle: u32,
}

#[error_code]
pub enum FarmErr {
    #[msg("Plot cell PDA mismatch")]
    Cell,
}
