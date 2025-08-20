use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("QuEsTtImE000000000000000000000000000004");

#[program]
pub mod quest_timeline {
    use super::*;

    pub fn start_arc(ctx: Context<StartArc>, name: Vec<u8>, bump: u8) -> Result<()> {
        let mut n = name.clone();
        if n.len() > 18 { n.truncate(18); }
        let mut code: u32 = 31;
        for b in n.iter() { code = code.wrapping_mul(97).wrapping_add(*b as u32); }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &n[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(QErr::Cell))?;
        if addr != ctx.accounts.arc_cell.key() { return Err(error!(QErr::Cell)); }

        let a = &mut ctx.accounts.arc;
        a.player = ctx.accounts.player.key();
        a.name = n;
        a.progress = 1;
        a.hash = code;
        Ok(())
    }

    pub fn advance_arc(ctx: Context<AdvanceArc>, step: u16, bump: u8) -> Result<()> {
        let tag = ctx.accounts.arc.name.clone();
        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(QErr::Cell))?;
        if addr != ctx.accounts.arc_cell.key() { return Err(error!(QErr::Cell)); }

        let a = &mut ctx.accounts.arc;
        let mut s = step as u32;
        if s > 5000 { s = 5000; }
        a.progress = a.progress.saturating_add(s);
        a.hash = a.hash.wrapping_add(113);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartArc<'info> {
    #[account(mut)]
    pub arc: Account<'info, Arc>,
    /// CHECK:
    pub arc_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AdvanceArc<'info> {
    #[account(mut)]
    pub arc: Account<'info, Arc>,
    /// CHECK:
    pub arc_cell: AccountInfo<'info>,
    pub player: AccountInfo<'info>,
}

#[account]
pub struct Arc {
    pub player: Pubkey,
    pub name: Vec<u8>,
    pub progress: u32,
    pub hash: u32,
}

#[error_code]
pub enum QErr {
    #[msg("Arc cell PDA mismatch")]
    Cell,
}
