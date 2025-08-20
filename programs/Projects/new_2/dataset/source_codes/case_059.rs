use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Burn, MintTo, burn, mint_to};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqNftFuse01");

#[program]
pub mod nft_fusion_example {
    use super::*;

    /// 2 枚の NFT をバーンして、新しい NFT をミントする  
    /// （`fusion_account` のオーナーチェックを省略しているため、  
    ///  攻撃者は任意のミントアカウントを指定し、不正に NFT を生成・消費できます）
    pub fn fuse_nfts(ctx: Context<FuseNfts>) -> Result<()> {
        // 1) NFT A をバーン
        let cpi_burn_a = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_a.to_account_info(),
                to:   ctx.accounts.token_account_a.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        burn(cpi_burn_a, 1)?;

        // 2) NFT B をバーン
        let cpi_burn_b = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint_b.to_account_info(),
                to:   ctx.accounts.token_account_b.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        burn(cpi_burn_b, 1)?;

        // 3) 新規 NFT をミント（fusion_account をミントアカウントとして使用）
        let cpi_mint = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint:      ctx.accounts.fusion_account.to_account_info(),      // owner チェック省略！
                to:        ctx.accounts.user_fusion_token.to_account_info(),
                authority: ctx.accounts.fusion_authority.to_account_info(),
            },
        );
        mint_to(cpi_mint, 1)?;

        msg!(
            "Fused NFTs {} + {} → minted 1 on {}",
            ctx.accounts.mint_a.key(),
            ctx.accounts.mint_b.key(),
            ctx.accounts.fusion_account.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FuseNfts<'info> {
    /// CHECK: mint_a の owner チェックを行っていない
    #[account(mut)] pub mint_a: AccountInfo<'info>,
    /// CHECK: mint_a のトークンアカウント（owner チェックなし）
    #[account(mut)] pub token_account_a: AccountInfo<'info>,

    /// CHECK: mint_b の owner チェックを行っていない
    #[account(mut)] pub mint_b: AccountInfo<'info>,
    /// CHECK: mint_b のトークンアカウント（owner チェックなし）
    #[account(mut)] pub token_account_b: AccountInfo<'info>,

    /// CHECK: fusion 用ミントアカウントの owner チェックを省略
    #[account(mut)] pub fusion_account: AccountInfo<'info>,
    /// CHECK: ユーザーのトークンアカウント（owner チェックなし）
    #[account(mut)] pub user_fusion_token: AccountInfo<'info>,

    /// NFT バーンとミントを実行するユーザー（署名のみ検証）
    pub user: Signer<'info>,
    /// このミントの権限を持つはずのアカウント（署名のみ検証）
    pub fusion_authority: Signer<'info>,

    /// SPL Token プログラム CPI 用
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("バーンやミントで失敗しました")] OperationFailed,
}
