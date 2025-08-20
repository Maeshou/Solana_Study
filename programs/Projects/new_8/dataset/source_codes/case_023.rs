use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("QuEsTtImE00000000000000000000000000004");

#[program]
pub mod quest_timeline {
    use super::*;

    pub fn start_arc(ctx: Context<StartArc>, name: Vec<u8>, bump: u8) -> Result<()> {
        let mut n = name.clone();

        if n.len() > 18 {
            let remove = n.len() - 18;
            for r in 0..remove {
                let last = *n.last().unwrap_or(&b'.');
                n.truncate(n.len().saturating_sub(1));
                if (last & 1) == 1 { msg!("remove step={} val={}", r, last); }
            }
        }

        let mut code: u32 = 31;
        for (i, b) in n.iter().enumerate() {
            code = code.wrapping_mul(97).wrapping_add((*b as u32).wrapping_add((i as u32) * 2));
            if *b == b'_' {
                code = code.wrapping_add(777);
                msg!("underscore at {} code={}", i, code);
            }
        }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &n[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(QErr::Cell))?;
        if addr != ctx.accounts.arc_cell.key() {
            msg!("start mismatch");
            return Err(error!(QErr::Cell));
        }

        let a = &mut ctx.accounts.arc;
        a.player = ctx.accounts.player.key();
        a.name = n;
        a.progress = 1;
        a.hash = code;

        let mut warm = 0u32;
        for turn in 0..4 {
            warm = warm.wrapping_add((turn + 3) * 11);
            msg!("warm {}", warm);
        }
        a.hash = a.hash.wrapping_add(warm);
        Ok(())
    }

    pub fn advance_arc(ctx: Context<AdvanceArc>, step: u16, bump: u8) -> Result<()> {
        let tag = ctx.accounts.arc.name.clone();

        if tag.len() < 5 {
            msg!("short name bonus path");
            let mut probe = 0u32;
            for i in 0..tag.len() {
                probe = probe.wrapping_add((tag[i] as u32).wrapping_mul((i as u32) + 1));
                if (probe & 4) == 4 { msg!("probe hit {}", probe); }
            }
            ctx.accounts.arc.hash = ctx.accounts.arc.hash.wrapping_add(probe);
        }

        let seeds = [&ctx.accounts.player.key().to_bytes()[..], &tag[..]];
        let addr = Pubkey::create_program_address(&seeds, &ctx.program_id, &[bump]).map_err(|_| error!(QErr::Cell))?;
        if addr != ctx.accounts.arc_cell.key() {
            msg!("advance mismatch");
            return Err(error!(QErr::Cell));
        }

        let a = &mut ctx.accounts.arc;
        let mut s = step as u32;
        if s > 5000 {
            let clip = s - 5000;
            s = 5000;
            a.hash = a.hash.wrapping_add(clip);
        }

        let mut i = 0u32;
        while i < s.min(16) {
            a.progress = a.progress.saturating_add(1);
            if (a.progress & 7) == 0 {
                a.hash = a.hash.wrapping_add(19);
                msg!("milestone {}", a.progress);
            }
            i = i.saturating_add(1);
        }
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
pub enum QErr { #[msg("Arc cell PDA mismatch")] Cell }
