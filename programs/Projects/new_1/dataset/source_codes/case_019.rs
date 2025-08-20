use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, TransferChecked};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfNEXT01");

#[program]
pub mod loyalty_distributor {
    use super::*;

    /// ユーザーにロイヤリティポイント（SPLトークン）を配布します。
    /// authority は AccountInfo のまま受け取り、署名チェックが行われません。
    pub fn distribute_points(
        ctx: Context<DistributePoints>,
        amount: u64,           // 配布するポイント数
    ) -> Result<()> {
        // CPI：SPLトークンの転送
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user_account.to_account_info(),
            authority: ctx.accounts.authority.clone(), // AccountInfo、署名チェックなし
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        // amount 個のトークンを転送
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!(
            "Distributed {} points to {}",
            amount,
            ctx.accounts.user.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DistributePoints<'info> {
    /// プログラムが管理するプール口座
    #[account(mut)]
    pub vault: Box<Account<'info, TokenAccount>>,

    /// ポイントを受け取るユーザーのトークン口座
    #[account(mut)]
    pub user_account: Box<Account<'info, TokenAccount>>,

    /// 配布権限を持つはずのアカウント（署名者チェックが行われない脆弱ポイント）
    pub authority: AccountInfo<'info>,

    /// 実際にポイントを受け取るユーザー
    #[account(signer)]
    pub user: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}
