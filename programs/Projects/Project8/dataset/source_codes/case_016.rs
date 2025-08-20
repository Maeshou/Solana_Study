// Program 5: esports_prize_bank （eスポーツ大会の賞金庫）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("EsP0rtsPrizeBank55555555555555555555555");

#[program]
pub mod esports_prize_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, season: u64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        b.host = ctx.accounts.host.key();
        b.season = season.rotate_left(2).wrapping_add(41);
        b.counter = 2;
        let mut t = 0u8;
        let mut z = b.season.rotate_right(1).wrapping_add(3);
        while t < 3 {
            z = z.rotate_left(1).wrapping_mul(2).wrapping_add(5);
            if z % 3 > 0 { b.counter = b.counter.saturating_add(((z % 29) as u32) + 1); }
            t = t.saturating_add(1);
        }
        Ok(())
    }

    pub fn payout_rounds(ctx: Context<PayoutRounds>, base: u64, rounds: u8) -> Result<()> {
        let bump = *ctx.bumps.get("bank").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"bank", ctx.accounts.host.key.as_ref(), &ctx.accounts.bank.season.to_le_bytes(), &[bump]];

        let mut i = 0u8;
        let mut amount = base.rotate_left(1).wrapping_add(13);
        while i < rounds {
            let ix = system_instruction::transfer(&ctx.accounts.bank.key(), &ctx.accounts.player.key(), amount);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.bank.to_account_info(),
                    ctx.accounts.player.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            amount = amount.rotate_right(1).wrapping_mul(3).wrapping_add(7);
            if amount % 2 > 0 { amount = amount.wrapping_add(5); }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(
        init,
        payer = host,
        space = 8 + 32 + 8 + 4,
        seeds=[b"bank", host.key().as_ref(), season.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub season: u64,
}

#[derive(Accounts)]
pub struct PayoutRounds<'info> {
    #[account(
        mut,
        seeds=[b"bank", host.key().as_ref(), bank.season.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bank {
    pub host: Pubkey,
    pub season: u64,
    pub counter: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
