use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgFarmYld001");

#[program]
pub mod farm_yield_service {
    use super::*;

    /// 農場から収穫を行い、報酬トークンを配布するが、
    /// farm_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn harvest_yield(
        ctx: Context<HarvestYield>,
        base_amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let farm = &mut ctx.accounts.farm_account;

        // 1. 収穫量を計算
        let amount = calculate_yield(config, base_amount);

        // 2. 農場アカウントに記録
        record_harvest(farm, amount);

        // 3. トークンをユーザーへ転送
        distribute_yield(ctx, amount)?;

        Ok(())
    }
}

/// 設定アカウントから乗数をかけて収穫量を計算するヘルパー
fn calculate_yield(config: &Account<FarmConfig>, base: u64) -> u64 {
    base
        .checked_mul(config.yield_multiplier)
        .unwrap()
}

/// 農場アカウントに収穫記録を残すヘルパー
fn record_harvest(farm: &mut FarmAccount, amount: u64) {
    farm.total_harvested = farm.total_harvested.saturating_add(amount);
    farm.harvest_count = farm.harvest_count.saturating_add(1);
}

/// CPI で報酬トークンを転送するヘルパー
fn distribute_yield(ctx: &Context<HarvestYield>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.yield_vault.to_account_info(),
        to: ctx.accounts.user_vault.to_account_info(),
        authority: ctx.accounts.service_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct HarvestYield<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して所有者照合を行うべき
    pub farm_account: Account<'info, FarmAccount>,

    /// 報酬トークンの保管アカウント
    #[account(mut)]
    pub yield_vault: Account<'info, TokenAccount>,

    /// ユーザーの受取用トークンアカウント
    #[account(mut)]
    pub user_vault: Account<'info, TokenAccount>,

    /// CPI 実行権限を持つサービスアカウント
    pub service_authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,

    /// 収穫倍率などを保持する設定アカウント
    pub config: Account<'info, FarmConfig>,
}

#[account]
pub struct FarmAccount {
    /// 本来この農場を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// これまでに収穫した合計量
    pub total_harvested: u64,
    /// 収穫実行回数
    pub harvest_count: u64,
}

#[account]
pub struct FarmConfig {
    /// 収穫時にかける乗数（例：2 = 2倍）
    pub yield_multiplier: u64,
}
