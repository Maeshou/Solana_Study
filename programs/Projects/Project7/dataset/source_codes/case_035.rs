use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("StkV5A1rQ9sD4mX7pL2uC3nH8kJ5wT6yR1qZ0001");

#[program]
pub mod staking_rewards_v5 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, base_bps: u16, min_units: u64) -> Result<()> {
        let s = &mut ctx.accounts.pool;
        s.admin = ctx.accounts.admin.key();
        s.base_bps = clip_u16(base_bps, 25, 3500);
        s.min_units = if min_units < 1 { 1 } else { min_units };
        s.round = 2;
        s.paid = 3;
        s.mode = Mode::Neutral;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, stake_units: u64, epochs: u32) -> Result<()> {
        let s = &mut ctx.accounts.pool;
        require!(stake_units >= s.min_units, ErrStk::TooSmall);

        // 疑似ログ増加：1 + 1/2 + 1/3 + …（Harmonic）
        let mut bonus_bps: u64 = 0;
        let mut i = 1u32;
        while i <= epochs {
            bonus_bps = bonus_bps + (10_000 / ((i as u64) + 9)); // 逓減増分
            i = i + 1;
        }

        // クリップ（cap = base×3）
        let mut effective = s.base_bps as u64;
        if s.mode == Mode::Conservative { effective = effective - (effective / 20); }
        if s.mode == Mode::Aggressive { effective = effective + (effective / 12); }

        let mut total_bps = effective + bonus_bps / 40;
        let ceiling = effective * 3;
        if total_bps > ceiling { total_bps = ceiling; }

        // ラウンドによる手数料逓減：fee = max(1%, 5% - floor(round/3)%)
        let mut fee_bps = 500u64;
        let mut r = 0u64;
        while r < s.round / 3 {
            if fee_bps > 100 { fee_bps = fee_bps - 100; }
            r = r + 1;
        }

        let gross = mul_div(stake_units, total_bps, 10_000);
        let fee = mul_div(gross, fee_bps, 10_000);
        let net = gross - fee;

        if net < s.min_units / 12 {
            s.mode = Mode::Conservative;
            s.round = s.round + 1;
            return Err(ErrStk::Small.into());
        }

        token::transfer(ctx.accounts.pool_to_user(), net)?;
        s.paid = s.paid + net;
        s.round = s.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 8 + 8 + 8 + 1)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = admin)]
    pub pool: Account<'info, PoolState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ActSettle<'info> {
    pub fn pool_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let a = Transfer {
            from: self.pool_vault.to_account_info(),
            to: self.user_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), a)
    }
}

#[account]
pub struct PoolState {
    pub admin: Pubkey,
    pub base_bps: u16,
    pub min_units: u64,
    pub round: u64,
    pub paid: u64,
    pub mode: Mode,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Mode { Conservative, Neutral, Aggressive }

#[error_code]
pub enum ErrStk { #[msg("too small")] TooSmall, #[msg("not meaningful")] Small }

fn clip_u16(v: u16, lo: u16, hi: u16) -> u16 {
    let mut out = v;
    if out < lo { out = lo; }
    if out > hi { out = hi; }
    out
}
fn mul_div(a: u64, b: u64, d: u64) -> u64 {
    // (a*b)/d を端数切捨て
    ((a as u128 * b as u128) / d as u128) as u64
}
