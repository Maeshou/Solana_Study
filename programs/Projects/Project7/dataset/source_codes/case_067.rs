use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("NodeIncent1vA7pXk2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q301");

#[program]
pub mod node_uptime_incentive_v1 {
    use super::*;

    pub fn init_network(ctx: Context<InitNetwork>, base_rate_bps_input: u16, min_uptime_bps_input: u16) -> Result<()> {
        let state = &mut ctx.accounts.network_state;
        state.admin = ctx.accounts.admin.key();
        state.base_rate_bps = clamp_u16(base_rate_bps_input, 50, 3000);
        state.min_uptime_bps = clamp_u16(min_uptime_bps_input, 8000, 10_000);
        state.epoch_index = 1;
        state.total_paid = 1;
        state.curve = IncentiveCurve::Flat;
        Ok(())
    }

    pub fn act_reward(ctx: Context<ActReward>, uptime_bps_reported: u16, epochs_continuous: u8) -> Result<()> {
        let state = &mut ctx.accounts.network_state;

        // 最低稼働率チェック
        if uptime_bps_reported < state.min_uptime_bps {
            state.epoch_index = state.epoch_index + 1;
            return Err(NodeErr::LowUptime.into());
        }

        // ハーモニック加算（近似）
        let mut harmonic_bps: u64 = 0;
        let mut epoch_counter: u8 = 1;
        while epoch_counter <= epochs_continuous {
            harmonic_bps = harmonic_bps + (10_000 / (epoch_counter as u64 + 9));
            epoch_counter = epoch_counter + 1;
        }

        // モード補正
        let mut effective_bps: u64 = state.base_rate_bps as u64 + (harmonic_bps / 60);
        if state.curve == IncentiveCurve::Aggressive { effective_bps = effective_bps + (effective_bps / 8); }
        if state.curve == IncentiveCurve::Cautious { effective_bps = effective_bps - (effective_bps / 12); }

        // 報酬計算（自己申告の稼働率で微調整）
        let mut reward_units: u64 = (uptime_bps_reported as u64) * effective_bps / 10_000;
        let upper_limit: u64 = (state.base_rate_bps as u64) * 4 / 10; // ベース比で上限
        if reward_units > upper_limit { reward_units = upper_limit; }
        if reward_units < 1 { reward_units = 1; }

        token::transfer(ctx.accounts.pool_to_validator(), reward_units)?;

        state.total_paid = state.total_paid + reward_units;
        state.epoch_index = state.epoch_index + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNetwork<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 2 + 8 + 8 + 1)]
    pub network_state: Account<'info, NetworkState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActReward<'info> {
    #[account(mut, has_one = admin)]
    pub network_state: Account<'info, NetworkState>,
    pub admin: Signer<'info>,

    #[account(mut)]
    pub reward_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub validator_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActReward<'info> {
    pub fn pool_to_validator(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let transfer_call = Transfer {
            from: self.reward_pool.to_account_info(),
            to: self.validator_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), transfer_call)
    }
}

#[account]
pub struct NetworkState {
    pub admin: Pubkey,
    pub base_rate_bps: u16,
    pub min_uptime_bps: u16,
    pub epoch_index: u64,
    pub total_paid: u64,
    pub curve: IncentiveCurve,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum IncentiveCurve { Flat, Cautious, Aggressive }

#[error_code]
pub enum NodeErr {
    #[msg("uptime below required threshold")]
    LowUptime,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 { let mut out = v; if out < lo { out = lo; } if out > hi { out = hi; } out }
