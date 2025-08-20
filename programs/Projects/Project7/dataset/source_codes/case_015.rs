use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("StkClrV3J7cP2m9JcP2m9JcP2m9JcP2m9JcP2m9");

#[program]
pub mod staking_rewards_clear_v3 {
    use super::*;

    pub fn init_pool(
        ctx: Context<InitPool>,
        base_bps: u16,
        min_units: u64,
        early_rounds: u8,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.admin = ctx.accounts.admin.key();
        pool.base_rate_bps = base_bps.min(2500).max(50);              // 最小50bps
        pool.minimum_stake = min_units.max(1);                        // 0回避
        pool.current_round = early_rounds.max(1) as u64;              // 1以上
        pool.cumulative_payout = min_units.saturating_div(3).max(1);  // 入力由来の初期値
        pool.conservative_mode = false;
        Ok(())
    }

    pub fn act_settle(
        ctx: Context<ActSettle>,
        deposit_units: u64,
        epochs: u16,
        boost_step_bps: u16,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        require!(deposit_units >= pool.minimum_stake, ErrStk::TooSmall);

        // ラウンド毎に逓減する手数料（初回は5%、以後1%ずつ減、下限1%）
        let mut fee_bps = 500u64;
        let mut r = 1u64;
        while r < pool.current_round {
            if fee_bps > 100 { fee_bps = fee_bps.saturating_sub(100); }
            r = r.saturating_add(1);
        }

        // エポックごとに加算されるブースト（上限はbaseの3倍）
        let mut boost_bps = 0u64;
        let mut e = 0u16;
        while e < epochs {
            boost_bps = boost_bps.saturating_add(boost_step_bps as u64);
            e = e.saturating_add(1);
        }
        let mut effective_bps = pool.base_rate_bps as u64;
        if pool.conservative_mode { effective_bps = effective_bps.saturating_sub(25); }
        let capped = effective_bps.saturating_add(boost_bps);
        let bps_for_reward = if capped > effective_bps.saturating_mul(3) {
            effective_bps.saturating_mul(3)
        } else {
            capped
        };

        let gross = deposit_units.saturating_mul(bps_for_reward) / 10_000;
        let fee = gross.saturating_mul(fee_bps) / 10_000;
        let net = gross.saturating_sub(fee);

        if net < pool.minimum_stake.saturating_div(12) {
            pool.conservative_mode = true;
            pool.current_round = pool.current_round.saturating_add(1);
            return Err(ErrStk::NoMeaningfulReward.into());
        }
        pool.cumulative_payout = pool.cumulative_payout.saturating_add(net);
        pool.current_round = pool.current_round.saturating_add(1);

        token::transfer(ctx.accounts.pool_to_staker(), net)?;
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
    pub fn pool_to_staker(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let accs = Transfer {
            from: self.pool_vault.to_account_info(),
            to: self.user_vault.to_account_info(),
            authority: self.admin.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), accs)
    }
}

#[account]
pub struct PoolState {
    pub admin: Pubkey,
    pub base_rate_bps: u16,
    pub minimum_stake: u64,
    pub current_round: u64,
    pub cumulative_payout: u64,
    pub conservative_mode: bool,
}

#[error_code]
pub enum ErrStk {
    #[msg("Stake amount below minimum")]
    TooSmall,
    #[msg("Reward is negligible")]
    NoMeaningfulReward,
}
