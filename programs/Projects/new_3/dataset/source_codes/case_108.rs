use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgVstVst001");

#[program]
pub mod vesting_service {
    use super::*;

    /// ベスティング済みトークンを解放するが、
    /// has_one = vesting_mint／has_one = authority はあるものの、
    /// beneficiary との一致チェックが欠如しているため、
    /// 攻撃者が他人のベネフィシアリーアカウントを指定して不正に解放可能
    pub fn release_vested(ctx: Context<ReleaseVested>) -> Result<()> {
        let vest = &mut ctx.accounts.vesting_account;
        // 1. 利用可能額を計算（total_amount - released）
        let amount = vest.total_amount.checked_sub(vest.released).unwrap();
        // 2. released を更新
        vest.released = vest.released.saturating_add(amount);
        // 3. Escrow Vault から beneficiary へトークンを転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_account.to_account_info(),
            to: ctx.accounts.beneficiary_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReleaseVested<'info> {
    #[account(
        mut,
        has_one = vesting_mint,
        has_one = authority,
        // 本来は has_one = beneficiary も指定して受取人検証を行うべき
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    /// ベスティング対象のトークン Mint
    pub vesting_mint: Account<'info, anchor_spl::token::Mint>,

    /// Escrow 用トークンアカウント
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,

    /// 解放を受け取るべきベネフィシアリーのトークンアカウント
    #[account(mut)]
    pub beneficiary_account: Account<'info, TokenAccount>,

    /// 解放権限を持つアカウント
    pub authority: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct VestingAccount {
    /// 本来このベスティング契約を所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// トークンを受け取るべきベネフィシアリーの Pubkey
    pub beneficiary: Pubkey,
    /// ベスティング対象トークンの Mint
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
