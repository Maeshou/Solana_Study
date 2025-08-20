use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("TrnP1izePoolA9K2M7Zq8X1L4D6W3Y5R2T0V9A101");

#[program]
pub mod tournament_prize_pool_v1 {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, base_award: u64, tier_boost_bps: u16) -> Result<()> {
        let state = &mut ctx.accounts.pool_state;
        state.organizer = ctx.accounts.organizer.key();
        state.base_award = if base_award < 2 { 2 } else { base_award };
        state.tier_boost_bps = clamp_u16(tier_boost_bps, 50, 2500);
        state.round = 1;
        state.total_distributed = base_award / 2 + 3;
        state.tier = PrizeTier::Silver;
        Ok(())
    }

    pub fn act_settle(
        ctx: Context<ActSettle>,
        participant_count: u8,
        extra_rounds: u8,
    ) -> Result<()> {
        let state = &mut ctx.accounts.pool_state;

        // 参加人数に応じた段階加算
        let mut tier_bonus: u64 = 0;
        let mut index: u8 = 1;
        while index <= participant_count {
            if index <= 4 { tier_bonus = tier_bonus + 10; }
            if index > 4 { tier_bonus = tier_bonus + 6; }
            if index > 8 { tier_bonus = tier_bonus + 3; }
            index = index + 1;
        }

        // 追加ラウンドの減衰ボーナス（1/2, 1/3, ...）
        let mut decay_bonus_bps: u64 = 0;
        let mut r: u8 = 1;
        while r <= extra_rounds {
            decay_bonus_bps = decay_bonus_bps + (10_000 / (r as u64 + 8));
            r = r + 1;
        }

        // ティア補正
        let mut effective_bps = state.tier_boost_bps as u64;
        if state.tier == PrizeTier::Bronze { effective_bps = effective_bps - (effective_bps / 10); }
        if state.tier == PrizeTier::Gold { effective_bps = effective_bps + (effective_bps / 8); }

        let mut award_units = state.base_award + tier_bonus + (decay_bonus_bps / 50);
        // 上限：ベースの3倍
        let ceiling = state.base_award * 3;
        if award_units > ceiling { award_units = ceiling; }
        // 下限
        if award_units < 1 { award_units = 1; }

        // 5%から始まり、ラウンド/2ごとに1%減（最小1%）
        let mut fee_bps: u64 = 500;
        let mut k: u64 = 0;
        while k < state.round / 2 {
            if fee_bps > 100 { fee_bps = fee_bps - 100; }
            k = k + 1;
        }

        let boosted = award_units + (award_units * effective_bps / 10_000);
        let fee_amount = boosted * fee_bps / 10_000;
        let payout_to_winner = boosted - fee_amount;

        if payout_to_winner < state.base_award / 10 {
            state.round = state.round + 1;
            return Err(PrizeErr::TooSmall.into());
        }

        token::transfer(ctx.accounts.treasury_to_winner(), payout_to_winner)?;
        token::transfer(ctx.accounts.treasury_to_fee(), fee_amount)?;

        state.total_distributed = state.total_distributed + payout_to_winner;
        state.round = state.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub pool_state: Account<'info, PrizePoolState>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = organizer)]
    pub pool_state: Account<'info, PrizePoolState>,
    pub organizer: Signer<'info>,

    #[account(mut)]
    pub prize_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettle<'info> {
    pub fn treasury_to_winner(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer {
            from: self.prize_treasury.to_account_info(),
            to: self.winner_vault.to_account_info(),
            authority: self.organizer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
    pub fn treasury_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let x = Transfer {
            from: self.prize_treasury.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.organizer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), x)
    }
}

#[account]
pub struct PrizePoolState {
    pub organizer: Pubkey,
    pub base_award: u64,
    pub tier_boost_bps: u16,
    pub round: u64,
    pub total_distributed: u64,
    pub tier: PrizeTier,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PrizeTier { Bronze, Silver, Gold }

#[error_code]
pub enum PrizeErr {
    #[msg("calculated prize too small")]
    TooSmall,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 {
    let mut out = v;
    if out < lo { out = lo; }
    if out > hi { out = hi; }
    out
}
