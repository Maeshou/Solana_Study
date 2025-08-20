use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgVstSvc01");

#[program]
pub mod vesting_service {
    use super::*;

    /// ベスティング済みトークンを解放するが、
    /// vesting_account.vesting_mint の照合のみで beneficiary の一致チェックがない
    pub fn release_vested(ctx: Context<ReleaseVested>) -> Result<()> {
        let vest = &mut ctx.accounts.vesting_account;
        // 利用可能額を計算（released を差し引いた残量）
        let amount = vest.total_amount.checked_sub(vest.released).unwrap();
        // 解放済み量を更新
        vest.released = vest.released.saturating_add(amount);
        // CPI でトークンを解放
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.beneficiary_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseVested<'info> {
    #[account(mut, has_one = vesting_mint, has_one = authority)]
    /// 本来は `has_one = beneficiary` も指定して解放受取人の一致照合を行うべき
    pub vesting_account: Account<'info, VestingAccount>,

    /// ベスティング対象のトークン Mint
    pub vesting_mint: Account<'info, Mint>,

    /// Escrow 用のトークンアカウント
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// ベネフィシアリーの受取用トークンアカウント
    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    /// 解放操作を実行する権限を持つアカウント
    pub authority: Signer<'info>,

    /// SPL トークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VestingAccount {
    /// 本来このベスティング契約を所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// トークンを受け取るべきベネフィシアリーの Pubkey
    pub beneficiary: Pubkey,
    /// ベスティング対象のトークン Mint
    pub vesting_mint: Pubkey,
    /// 総ベスティング量
    pub total_amount: u64,
    /// 既に解放された量
    pub released: u64,
    /// 開始時刻 (UNIXタイム)
    pub start_time: i64,
    /// クリフ期間 (秒)
    pub cliff_duration: i64,
    /// ベスティング期間 (秒)
    pub vesting_duration: i64,
}
