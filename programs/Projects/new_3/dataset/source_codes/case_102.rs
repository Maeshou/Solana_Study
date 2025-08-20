use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, MintTo, CpiContext, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgFractSvc01");

#[program]
pub mod fractionalize_service {
    use super::*;

    /// NFT をフラクショナル化して分割トークンをミントするが、
    /// FractionAccount.owner の照合チェックがなく、攻撃者は他人のアカウントで実行可能
    pub fn fractionalize(
        ctx: Context<Fractionalize>,
        fraction_amount: u64,
    ) -> Result<()> {
        let frac_acc = &mut ctx.accounts.fraction_account;

        // 1. 累計分割トークン数を更新
        frac_acc.total_fractions = frac_acc
            .total_fractions
            .checked_add(fraction_amount)
            .unwrap();

        // 2. CPI で分割トークンをユーザーにミント
        let cpi_accounts = MintTo {
            mint: ctx.accounts.fraction_mint.to_account_info(),
            to: ctx.accounts.user_fraction_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, fraction_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fractionalize<'info> {
    #[account(
        mut,
        has_one = fraction_mint,  // フラクションミントだけ検証
        // 本来は has_one = owner を指定して所有者照合を行うべき
    )]
    pub fraction_account: Account<'info, FractionAccount>,

    /// 分割トークンの Mint
    pub fraction_mint: Account<'info, Mint>,

    /// ユーザーの分割トークン受取用 TokenAccount
    #[account(mut)]
    pub user_fraction_account: Account<'info, TokenAccount>,

    /// Mint 権限を持つアカウント
    pub mint_authority: Signer<'info>,

    /// SPL Token プログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct FractionAccount {
    /// 本来このフラクショナル化を行うべきユーザーの Pubkey
    pub owner: Pubkey,

    /// 対応する分割トークンの Mint アドレス
    pub fraction_mint: Pubkey,

    /// これまでにミントされた分割トークンの総数
    pub total_fractions: u64,
}
