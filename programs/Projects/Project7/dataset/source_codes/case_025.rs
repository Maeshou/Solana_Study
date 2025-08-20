use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("StkV4u9mH3pQ7r2Zs8X4c1N0kAaBbCcDdEeFf001");

#[program]
pub mod staking_rewards_v4 {
    use super::*;

    pub fn init_pool(
        ctx: Context<InitPool>,
        base_rate_bps: u16,
        minimum_units: u64,
        warmup_rounds: u8,
    ) -> Result<()> {
        let pool_info = &mut ctx.accounts.pool_info;
        pool_info.admin_key = ctx.accounts.admin.key();
        pool_info.base_rate_bps = base_rate_bps.min(3000).max(25);
        pool_info.minimum_units = minimum_units.max(1);
        pool_info.current_round = (warmup_rounds as u64).max(1);
        pool_info.cumulative_distributed = minimum_units.saturating_add(3);
        pool_info.mode_flag = RewardMode::Balanced;
        Ok(())
    }

    pub fn act_settle(
        ctx: Context<ActSettle>,
        contributed_units: u64,
        epoch_count: u16,
        ramp_bps_per_epoch: u16,
    ) -> Result<()> {
        let pool_info = &mut ctx.accounts.pool_info;
        require!(contributed_units >= pool_info.minimum_units, ErrStk::TooSmall);

        // 手数料はラウンド進行とともに逓減（下限1%）
        let mut fee_bps = 400u64;
        let mut round_cursor = 1u64;
        while round_cursor < pool_info.current_round {
            if fee_bps > 100 { fee_bps = fee_bps.saturating_sub(50); }
            round_cursor = round_cursor.saturating_add(1);
        }

        // 逓増係数（capはbase×3）
        let mut accumulated_bonus_bps = 0u64;
        let mut epoch_cursor = 0u16;
        while epoch_cursor < epoch_count {
            accumulated_bonus_bps = accumulated_bonus_bps.saturating_add(ramp_bps_per_epoch as u64);
            epoch_cursor = epoch_cursor.saturating_add(1);
        }

        let mut effective_bps = pool_info.base_rate_bps as u64;
        if pool_info.mode_flag == RewardMode::Cautious { effective_bps = effective_bps.saturating_sub(30); }
        if pool_info.mode_flag == RewardMode::Focused { effective_bps = effective_bps.saturating_add(90); }

        let capped_bps = {
            let candidate = effective_bps.saturating_add(accumulated_bonus_bps);
            let ceiling = effective_bps.saturating_mul(3);
            if candidate > ceiling { ceiling } else { candidate }
        };

        let gross_reward = contributed_units.saturating_mul(capped_bps) / 10_000;
        let fee_amount = gross_reward.saturating_mul(fee_bps) / 10_000;
        let distributable = gross_reward.saturating_sub(fee_amount);

        if distributable < pool_info.minimum_units.saturating_div(8) {
            pool_info.mode_flag = RewardMode::Cautious;
            pool_info.current_round = pool_info.current_round.saturating_add(1);
            return Err(ErrStk::InsufficientYield.into());
        }

        token::transfer(ctx.accounts.pool_to_staker(), distributable)?;
        pool_info.cumulative_distributed = pool_info.cumulative_distributed.saturating_add(distributable);
        pool_info.current_round = pool_info.current_round.saturating_add(1);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 8 + 8 + 8 + 1)]
    pub pool_info: Account<'info, PoolInfo>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = admin_key)]
    pub pool_info: Account<'info, PoolInfo>,
    pub admin_key: Signer<'info>,

    #[account(mut)]
    pub pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staker_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ActSettle<'info> {
    pub fn pool_to_staker(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accounts = Transfer {
            from: self.pool_vault.to_account_info(),
            to: self.staker_vault.to_account_info(),
            authority: self.admin_key.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accounts)
    }
}

#[account]
pub struct PoolInfo {
    pub admin_key: Pubkey,
    pub base_rate_bps: u16,
    pub minimum_units: u64,
    pub current_round: u64,
    pub cumulative_distributed: u64,
    pub mode_flag: RewardMode,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RewardMode {
    Cautious,
    Balanced,
    Focused,
}

#[error_code]
pub enum ErrStk {
    #[msg("Amount below requirement")]
    TooSmall,
    #[msg("Distribution is not meaningful")]
    InsufficientYield,
}
