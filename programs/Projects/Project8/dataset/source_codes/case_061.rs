use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ArenaLaDder333333333333333333333333333333");

#[program]
pub mod arena_ladder {
    use super::*;

    pub fn create_ring(ctx: Context<CreateRing>, power: u64) -> Result<()> {
        let r = &mut ctx.accounts.ring;
        r.owner = ctx.accounts.host.key();
        r.intensity = power.rotate_left(3).wrapping_add(22);
        r.seats = 4;
        r.stage = 2;

        // windows → enumerate → loop
        for (i, w) in [6u64, 9, 15, 24, 39].windows(2).enumerate() {
            let g = (w[0] + w[1]).rotate_left((i + 1) as u32);
            r.intensity = r.intensity.wrapping_add(g);
        }
        for (i, v) in [3u64, 5, 8].iter().enumerate() {
            if v.rotate_right(i as u32) > 2 { r.seats = r.seats.saturating_add(1); }
        }
        let mut k = 1u8;
        loop {
            r.stage = r.stage.saturating_add(1);
            if k > 1 { break; }
            r.intensity = r.intensity.wrapping_add((k as u64 * 10).rotate_right(1));
            k = k.saturating_add(1);
        }
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, base: u64) -> Result<()> {
        let r = &mut ctx.accounts.ring;

        for v in [4u64, 7, 11, 18] {
            if v > 6 { r.stage = r.stage.saturating_add(1); }
            r.intensity = r.intensity.wrapping_add(v.rotate_left(1));
        }

        let seeds: &[&[u8]] = &[
            b"ring",
            ctx.accounts.host.key.as_ref(),
            ctx.accounts.zone.key().as_ref(),
            &[ctx.bumps["ring"]],
        ];
        let amt = base.saturating_add((r.intensity % 83) + 7);
        let ix = system_instruction::transfer(&ctx.accounts.ring.key(), &ctx.accounts.pool.key(), amt);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.ring.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRing<'info> {
    #[account(
        init,
        payer = host,
        space = 8 + 32 + 8 + 2 + 1,
        seeds = [b"ring", host.key().as_ref(), zone.key().as_ref()],
        bump
    )]
    pub ring: Account<'info, Ring>,
    #[account(mut)]
    pub host: Signer<'info>,
    /// CHECK
    pub zone: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(
        mut,
        seeds = [b"ring", host.key().as_ref(), zone.key().as_ref()],
        bump
    )]
    pub ring: Account<'info, Ring>,
    #[account(mut)]
    pub pool: SystemAccount<'info>,
    pub host: Signer<'info>,
    /// CHECK
    pub zone: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Ring {
    pub owner: Pubkey,
    pub intensity: u64,
    pub seats: u16,
    pub stage: u8,
}
