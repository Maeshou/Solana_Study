use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgSwapSvc01");

#[program]
pub mod token_swap_service {
    use super::*;

    /// ユーザーが TokenA を TokenB と交換するが、
    /// swap_account.owner の照合チェックがなく、攻撃者が他人のスワップアカウントを指定して操作できる
    pub fn swap_tokens(ctx: Context<SwapTokens>, amount: u64) -> Result<()> {
        // 1. スワップ回数をインクリメント
        let swap_acc = &mut ctx.accounts.swap_account;
        swap_acc.swap_count = swap_acc.swap_count.saturating_add(1);

        // 2. ユーザーの TokenA からプールの VaultA へ転送
        let cpi_in = Transfer {
            from: ctx.accounts.user_source.to_account_info(),
            to: ctx.accounts.vault_source.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx_in = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_in);
        token::transfer(cpi_ctx_in, amount)?;

        // 3. プールの VaultB からユーザーの TokenB へ転送（手数料差し引き）
        let out_amount = amount.checked_sub(ctx.accounts.config.swap_fee).unwrap();
        let cpi_out = Transfer {
            from: ctx.accounts.vault_dest.to_account_info(),
            to: ctx.accounts.user_dest.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx_out = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_out);
        token::transfer(cpi_ctx_out, out_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut, has_one = pool)]
    /// 本来は `has_one = owner` も追加して
    /// `swap_account.owner` と `ctx.accounts.user.key()` の一致を検証すべき
    pub swap_account: Account<'info, SwapAccount>,

    /// ユーザーの TokenA 保有アカウント
    #[account(mut)]
    pub user_source: Account<'info, TokenAccount>,

    /// プールの TokenA Vault
    #[account(mut)]
    pub vault_source: Account<'info, TokenAccount>,

    /// プールの TokenB Vault
    #[account(mut)]
    pub vault_dest: Account<'info, TokenAccount>,

    /// ユーザーの TokenB 受取アカウント
    #[account(mut)]
    pub user_dest: Account<'info, TokenAccount>,

    /// 関連する流動性プール
    pub pool: Account<'info, SwapPool>,

    /// スワップ時の手数料設定
    pub config: Account<'info, SwapConfig>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,

    /// スワップをリクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct SwapAccount {
    /// 本来このスワップを所有すべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 紐づく SwapPool の Pubkey
    pub pool: Pubkey,
    /// 実行されたスワップ回数
    pub swap_count: u64,
}

#[account]
pub struct SwapConfig {
    /// 1 回のスワップで徴収する手数料 (TokenA 単位)
    pub swap_fee: u64,
}

#[account]
pub struct SwapPool {
    /// プール内の TokenA Vault アドレス
    pub token_a_vault: Pubkey,
    /// プール内の TokenB Vault アドレス
    pub token_b_vault: Pubkey,
    /// 紐づく SwapAccount のアドレス
    pub swap_account: Pubkey,
}
