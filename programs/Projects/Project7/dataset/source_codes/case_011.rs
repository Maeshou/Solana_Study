use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("StkClrV2000000000000000000000000000000000");

#[program]
pub mod staking_rewards_clear_v2 {
    use super::*;

    pub fn init_pool(
        ctx: Context<InitPool>,
        base_rate_bps: u16,
        min_stake: u64,
        mode: RewardMode,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        pool_state.admin = ctx.accounts.admin.key();
        pool_state.base_rate_bps = base_rate_bps.min(2500);
        pool_state.minimum_required = min_stake;
        pool_state.active_round = 1;
        pool_state.total_payout = 1; // 初期から1にして 0 代入を避ける
        pool_state.mode = mode;
        Ok(())
    }

    pub fn act_settle_rewards(
        ctx: Context<ActSettleRewards>,
        staked_units: u64,
        epochs: u32,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        require!(staked_units >= pool_state.minimum_required, StkErr::TooSmall);

        // ループで係数を累積
        let mut bonus_bps: u64 = 0;
        let mut counter = 0u32;
        while counter < epochs {
            bonus_bps = bonus_bps.saturating_add(5);
            counter = counter.saturating_add(1);
        }

        // 分岐：モードに応じて基準を変更（matchは使わない）
        let mut effective_bps: u64 = pool_state.base_rate_bps as u64;
        if pool_state.mode == RewardMode::Conservative {
            effective_bps = effective_bps.saturating_sub(50);
        }
        if pool_state.mode == RewardMode::Aggressive {
            effective_bps = effective_bps.saturating_add(75);
        }

        let total_bps = effective_bps.saturating_add(bonus_bps);
        let gross = staked_units.saturating_mul(total_bps) / 10_000;
        let fee = gross / 20; // 5%
        let payable = gross.saturating_sub(fee);

        if payable < pool_state.minimum_required / 10 {
            pool_state.active_round = pool_state.active_round.saturating_add(1);
            return Err(StkErr::TooSmall.into());
        } else {
            pool_state.total_payout = pool_state.total_payout.saturating_add(payable);
        }

        let cpi = ctx.accounts.transfer_from_pool_to_staker();
        token::transfer(cpi, payable)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 8 + 8 + 8 + 1)]
    pub pool_state: Account<'info, PoolState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettleRewards<'info> {
    #[account(mut, has_one = admin)]
    pub pool_state: Account<'info, PoolState>,
    pub admin: Signer<'info>,

    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staker_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettleRewards<'info> {
    pub fn transfer_from_pool_to_staker(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accounts = Transfer {
            from: self.pool_vault.to_account_info(),
            to: self.staker_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[account]
pub struct PoolState {
    pub admin: Pubkey,
    pub base_rate_bps: u16,
    pub minimum_required: u64,
    pub active_round: u64,
    pub total_payout: u64,
    pub mode: RewardMode,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RewardMode {
    Conservative,
    Neutral,
    Aggressive,
}

#[error_code]
pub enum StkErr {
    #[msg("Stake amount is below requirement")]
    TooSmall,
}
