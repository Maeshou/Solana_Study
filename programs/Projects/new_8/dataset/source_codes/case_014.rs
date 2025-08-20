use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("EnErGyFoRgE0000000000000000000000000002");

#[program]
pub mod energy_forge {
    use super::*;

    pub fn infuse_crystal(ctx: Context<InfuseCrystal>, label: Vec<u8>, amount: u64, bump: u8) -> Result<()> {
        let mut l = label.clone();
        if l.is_empty() { l.extend_from_slice(b"seed"); }
        if l.len() > 28 { l.truncate(28); }
        let mut hash: u64 = 1;
        for c in l.iter() { hash = hash.wrapping_mul(257).wrapping_add(*c as u64); }

        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &l[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(FErr::Cell))?;
        if addr != ctx.accounts.crystal_cell.key() { return Err(error!(FErr::Cell)); }

        let mut a = amount;
        if a > 80_000 { a = 80_000; }
        let c = &mut ctx.accounts.crystal;
        c.owner = ctx.accounts.owner.key();
        c.label = l;
        c.energy = c.energy.saturating_add(a);
        c.trace = c.trace.wrapping_add(hash);
        Ok(())
    }

    pub fn consume_crystal(ctx: Context<ConsumeCrystal>, cost: u32, bump: u8) -> Result<()> {
        let tag = ctx.accounts.crystal.label.clone();
        let seeds = [&ctx.accounts.owner.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(FErr::Cell))?;
        if addr != ctx.accounts.crystal_cell.key() { return Err(error!(FErr::Cell)); }

        let c = &mut ctx.accounts.crystal;
        let mut used = cost as u64;
        if used > c.energy { used = c.energy; }
        c.energy = c.energy.saturating_sub(used);
        c.trace = c.trace.wrapping_add(13);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InfuseCrystal<'info> {
    #[account(mut)]
    pub crystal: Account<'info, Crystal>,
    /// CHECK:
    pub crystal_cell: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConsumeCrystal<'info> {
    #[account(mut)]
    pub crystal: Account<'info, Crystal>,
    /// CHECK:
    pub crystal_cell: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
}

#[account]
pub struct Crystal {
    pub owner: Pubkey,
    pub label: Vec<u8>,
    pub energy: u64,
    pub trace: u64,
}

#[error_code]
pub enum FErr {
    #[msg("Crystal cell PDA mismatch")]
    Cell,
}
