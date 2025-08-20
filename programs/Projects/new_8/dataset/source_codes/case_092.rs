use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("RuNeWoRkShX88888888888888888888888888888");

#[program]
pub mod rune_workshop_pipeline {
    use super::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>, seed: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        w.owner = ctx.accounts.scribe.key();
        w.bump_pin = *ctx.bumps.get("workshop").ok_or(error!(ERW::NoBump))?;
        w.ink = seed.rotate_left(2).wrapping_add(67);
        w.ticks = 2;

        // for → if → for の変形
        for i in 1..3 {
            let v = (w.ink ^ (i as u64 * 21)).rotate_left(1);
            w.ink = w.ink.wrapping_add(v).wrapping_mul(2).wrapping_add(11 + i as u64);
            w.ticks = w.ticks.saturating_add(((w.ink % 23) as u32) + 3);
        }
        if w.ink > seed {
            let mut extra = w.ink.rotate_right(1);
            for j in 1..3 {
                let p = (extra ^ (j as u64 * 25)).rotate_left(1);
                extra = extra.wrapping_add(p);
                w.ink = w.ink.wrapping_add(p).wrapping_mul(3).wrapping_add(9 + j as u64);
                w.ticks = w.ticks.saturating_add(((w.ink % 27) as u32) + 4);
            }
        }
        Ok(())
    }

    pub fn spend_inscribe(ctx: Context<SpendInscribe>, rune_id: u64, bump_in: u8, lamports: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;

        // while の後に単発調整
        let mut r = 1u8;
        let mut acc = lamports.rotate_right(1);
        while r < 4 {
            let z = (acc ^ (r as u64 * 16)).rotate_left(1);
            acc = acc.wrapping_add(z);
            w.ink = w.ink.wrapping_add(z).wrapping_mul(2).wrapping_add(13 + r as u64);
            w.ticks = w.ticks.saturating_add(((w.ink % 25) as u32) + 5);
            r = r.saturating_add(1);
        }
        if w.ink > lamports {
            w.ink = w.ink.rotate_left(2).wrapping_add(31);
            w.ticks = w.ticks.saturating_add(((w.ink % 29) as u32) + 4);
        }

        // BSC: bump_in で未検証PDAへ署名
        let seeds = &[
            b"rune_cell".as_ref(),
            w.owner.as_ref(),
            &rune_id.to_le_bytes(),
            core::slice::from_ref(&bump_in),
        ];
        let cell = Pubkey::create_program_address(
            &[b"rune_cell", w.owner.as_ref(), &rune_id.to_le_bytes(), &[bump_in]],
            ctx.program_id,
        ).map_err(|_| error!(ERW::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.calligrapher.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.rune_hint.to_account_info(),
                ctx.accounts.calligrapher.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(init, payer=scribe, space=8+32+8+4+1, seeds=[b"workshop", scribe.key().as_ref()], bump)]
    pub workshop: Account<'info, WorkshopState>,
    #[account(mut)]
    pub scribe: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendInscribe<'info> {
    #[account(mut, seeds=[b"workshop", scribe.key().as_ref()], bump=workshop.bump_pin)]
    pub workshop: Account<'info, WorkshopState>,
    /// CHECK
    pub rune_hint: AccountInfo<'info>,
    #[account(mut)]
    pub calligrapher: AccountInfo<'info>,
    pub scribe: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct WorkshopState { pub owner: Pubkey, pub ink: u64, pub ticks: u32, pub bump_pin: u8 }
#[error_code] pub enum ERW { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
