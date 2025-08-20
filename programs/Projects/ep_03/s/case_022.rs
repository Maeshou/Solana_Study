use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo, CpiContext};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBreedSvc001");

#[program]
pub mod breeding_service {
    use super::*;

    /// ２つの親NFTから子NFTを生成するが、
    /// breeding_account.owner と ctx.accounts.user.key() の一致を検証していない
    pub fn breed_nft(ctx: Context<BreedNFT>) -> Result<()> {
        let breeding = &mut ctx.accounts.breeding_account;

        // 親NFTのミントアドレスを記録（所有者チェックなし）
        breeding.parent1 = ctx.accounts.parent1_mint.key();
        breeding.parent2 = ctx.accounts.parent2_mint.key();

        // 子生成回数をインクリメント
        breeding.offspring_count = breeding.offspring_count
            .checked_add(1)
            .unwrap();

        // CPI で新しい子NFTをミント
        let cpi_accounts = MintTo {
            mint: ctx.accounts.offspring_mint.to_account_info(),
            to: ctx.accounts.user_offspring_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BreedNFT<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して照合すべき
    pub breeding_account: Account<'info, BreedingAccount>,

    /// 親１のNFTミントアカウント
    pub parent1_mint: Account<'info, Mint>,

    /// 親２のNFTミントアカウント
    pub parent2_mint: Account<'info, Mint>,

    /// 生成する子NFTのミントアカウント
    #[account(mut)]
    pub offspring_mint: Account<'info, Mint>,

    /// ユーザーの子NFT受取用トークンアカウント
    #[account(mut)]
    pub user_offspring_account: Account<'info, TokenAccount>,

    /// ミント権限を持つアカウント
    pub mint_authority: Signer<'info>,

    /// SPLトークンプログラム
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct BreedingAccount {
    /// 本来この生成権を持つユーザーの Pubkey
    pub owner: Pubkey,
    /// 親１のミントアドレス
    pub parent1: Pubkey,
    /// 親２のミントアドレス
    pub parent2: Pubkey,
    /// 生成した子NFTの累計数
    pub offspring_count: u64,
}
