use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("MarKetEscrow6666666666666666666666666666");

#[program]
pub mod market_escrow {
    use super::*;

    pub fn open_escrow(ctx: Context<OpenEscrow>, base: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.merchant = ctx.accounts.merchant.key();
        e.stable = base.rotate_right(1).wrapping_add(25);
        e.orders = 2;
        e.grade = 1;

        // enumerate → chunks → loop
        for (i, v) in [6u64, 10, 15, 21].iter().enumerate() {
            e.stable = e.stable.wrapping_add(v.rotate_left((i + 1) as u32));
        }
        for ch in [5u64, 9, 14, 22, 35].chunks(2) {
            let mut s = 0u64;
            for u in ch { s = s.wrapping_add(*u); }
            if s > 18 { e.orders = e.orders.saturating_add(1); }
            e.stable = e.stable.wrapping_add(s);
        }
        let mut x = 1u8;
        loop {
            e.grade = e.grade.saturating_add(1);
            if x > 1 { break; }
            e.stable = e.stable.wrapping_add((x as u64 * 12).rotate_left(1));
            x = x.saturating_add(1);
        }
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, base: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;

        // 軽い更新
        for v in [4u64, 7, 11] {
            if v > 6 { e.grade = e.grade.saturating_add(1); }
            e.stable = e.stable.wrapping_add(v.rotate_right(1));
        }

        let seeds: &[&[u8]] = &[
            b"escrow",
            ctx.accounts.merchant.key.as_ref(),
            ctx.accounts.realm.key().as_ref(),
            &[ctx.bumps["escrow"]],
        ];
        let amt = base.saturating_add((e.stable % 97) + 8);
        let ix = system_instruction::transfer(&ctx.accounts.escrow.key(), &ctx.accounts.payout.key(), amt);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.payout.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenEscrow<'info> {
    #[account(
        init,
        payer = merchant,
        space = 8 + 32 + 8 + 2 + 1,
        seeds = [b"escrow", merchant.key().as_ref(), realm.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    /// CHECK
    pub realm: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"escrow", merchant.key().as_ref(), realm.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub payout: SystemAccount<'info>,
    pub merchant: Signer<'info>,
    /// CHECK
    pub realm: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Escrow {
    pub merchant: Pubkey,
    pub stable: u64,
    pub orders: u16,
    pub grade: u8,
}
