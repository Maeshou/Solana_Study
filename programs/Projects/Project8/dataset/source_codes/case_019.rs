// Program 8: loyalty_points_bank （ロイヤルティポイント銀行）
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("L0yaltyPointsBank8888888888888888888888");

#[program]
pub mod loyalty_points_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, series: u64) -> Result<()> {
        let b = &mut ctx.accounts.bank;
        b.owner = ctx.accounts.owner.key();
        b.series = series.rotate_left(2).wrapping_add(25);
        b.level = 1;
        for _ in 0..3 {
            b.level = b.level.saturating_add(((b.series % 19) as u32) + 1);
        }
        Ok(())
    }

    pub fn award(ctx: Context<Award>, points: u64) -> Result<()> {
        let bump = *ctx.bumps.get("bank").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"bank", ctx.accounts.owner.key.as_ref(), &ctx.accounts.bank.series.to_le_bytes(), &[bump]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.bank_token.to_account_info(),
            to: ctx.accounts.user_token.to_account_info(),
            authority: ctx.accounts.bank.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, &[seeds]);

        // ポイント付与を段階的に行う
        let mut rounds = 3u8;
        let mut step = (points / 3).max(1);
        let mut left = points;
        while rounds > 0 {
            let give = left.min(step);
            token::transfer(cpi_ctx.clone(), give)?;
            left = left.saturating_sub(give);
            step = step.rotate_left(1).wrapping_add(1);
            if left > 0 && step > left { step = left; }
            rounds = rounds.saturating_sub(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4,
        seeds=[b"bank", owner.key().as_ref(), series.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub series: u64,
}

#[derive(Accounts)]
pub struct Award<'info> {
    #[account(
        mut,
        seeds=[b"bank", owner.key().as_ref(), bank.series.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankState>,
    #[account(mut)]
    pub bank_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub owner: Signer<'info>,
}

#[account]
pub struct BankState {
    pub owner: Pubkey,
    pub series: u64,
    pub level: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
