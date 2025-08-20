use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("AllyA01GrantX9Q7Lm3R8tD6W4yZ1nC5bK2hU0P301");

#[program]
pub mod alliance_grant_v1 {
    use super::*;

    pub fn init_alliance(ctx: Context<InitAlliance>, base_units_input: u64, start_fee_bps: u16) -> Result<()> {
        let state = &mut ctx.accounts.alliance;
        state.admin = ctx.accounts.admin.key();
        state.base_units = base_units_input;
        if state.base_units < 1 { state.base_units = 1; }
        state.fee_bps = clamp_u16(start_fee_bps, 100, 2500);
        state.round_index = 1;
        state.total_distributed = 1;
        state.scale = AllianceScale::Mid;
        Ok(())
    }

    pub fn act_grant(ctx: Context<ActGrant>, member_count: u8, sessions: u8) -> Result<()> {
        let state = &mut ctx.accounts.alliance;

        // メンバー数で段階加点
        let mut member_bonus: u64 = 0;
        let mut cursor: u8 = 1;
        while cursor <= member_count {
            if cursor <= 4 { member_bonus = member_bonus + 7; }
            if cursor > 4 { member_bonus = member_bonus + 4; }
            if cursor > 9 { member_bonus = member_bonus + 3; }
            cursor = cursor + 1;
        }

        // セッションのハーモニック加算
        let mut harmonic_bps: u64 = 0;
        let mut step: u8 = 1;
        while step <= sessions {
            harmonic_bps = harmonic_bps + (10_000 / (step as u64 + 9));
            step = step + 1;
        }

        // スケール補正
        let mut effective = state.base_units + (harmonic_bps / 50);
        if state.scale == AllianceScale::High { effective = effective + effective / 8; }
        if state.scale == AllianceScale::Low { effective = effective - effective / 12; }
        effective = effective + member_bonus;

        // フィー逓減（最小1%）
        let mut fee_bps_now: u64 = state.fee_bps as u64;
        let mut r: u64 = 0;
        while r < state.round_index / 2 {
            if fee_bps_now > 100 { fee_bps_now = fee_bps_now - 100; }
            r = r + 1;
        }

        // 上限：base×4
        let cap = state.base_units * 4;
        if effective > cap { effective = cap; }
        if effective < 1 { effective = 1; }

        let fee_units = (effective as u128 * fee_bps_now as u128 / 10_000u128) as u64;
        let net_units = effective - fee_units;

        token::transfer(ctx.accounts.treasury_to_member(), net_units)?;
        token::transfer(ctx.accounts.treasury_to_fee(), fee_units)?;

        state.total_distributed = state.total_distributed + net_units;
        state.round_index = state.round_index + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAlliance<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 2 + 8 + 8 + 1)]
    pub alliance: Account<'info, AllianceState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActGrant<'info> {
    #[account(mut, has_one = admin)]
    pub alliance: Account<'info, AllianceState>,
    pub admin: Signer<'info>,

    #[account(mut)]
    pub reward_treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub member_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActGrant<'info> {
    pub fn treasury_to_member(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.reward_treasury.to_account_info(),
            to: self.member_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
    pub fn treasury_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let t = Transfer {
            from: self.reward_treasury.to_account_info(),
            to: self.fee_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), t)
    }
}

#[account]
pub struct AllianceState {
    pub admin: Pubkey,
    pub base_units: u64,
    pub fee_bps: u16,
    pub round_index: u64,
    pub total_distributed: u64,
    pub scale: AllianceScale,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum AllianceScale { Low, Mid, High }

fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o}
