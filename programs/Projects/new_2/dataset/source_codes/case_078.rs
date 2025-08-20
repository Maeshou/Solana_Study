use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, MintTo, Token, burn, mint_to};

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqSplitMerge01");

#[program]
pub mod nft_split_merge {
    use super::*;

    /// NFT をバーンして、分割用トークントークンをミントする  
    /// （`source_nft_account` および `fraction_mint` のオーナーチェックを省略しているため、
    ///  攻撃者が他人の NFT アカウントや任意のミントを指定して
    ///  不正に分割トークンを発行できます）
    pub fn split_nft(
        ctx: Context<SplitNft>,
        fraction_amount: u64,  // 分割後にミントするフラクショントークン枚数
    ) -> Result<()> {
        // 1) 元 NFT を強制バーン（owner チェックなし）
        let cpi_burn = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.source_mint.to_account_info(),
                to:        ctx.accounts.source_nft_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        burn(cpi_burn, 1)?;

        // 2) 分割用トークンをミント
        let cpi_mint = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint:      ctx.accounts.fraction_mint.to_account_info(),     // owner チェックなし
                to:        ctx.accounts.fraction_account.to_account_info(),
                authority: ctx.accounts.fraction_authority.to_account_info(),
            },
        );
        mint_to(cpi_mint, fraction_amount)?;

        msg!(
            "Split NFT {} into {} fraction tokens on {}",
            ctx.accounts.source_nft_account.key(),
            fraction_amount,
            ctx.accounts.fraction_mint.key(),
        );
        Ok(())
    }

    /// 分割トークンをバーンして、NFT を再ミントする  
    /// （`fraction_account` および `target_mint` のオーナーチェックを省略しているため、
    ///  攻撃者が他人の分割トークンやミントを指定して
    ///  不正にNFTを再生成できます）
    pub fn merge_nft(
        ctx: Context<MergeNft>,
        required_fractions: u64,  // 必要なフラクショントークン枚数
    ) -> Result<()> {
        // 1) 分割トークンをバーン
        let cpi_burn = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.fraction_mint.to_account_info(),
                to:        ctx.accounts.fraction_account.to_account_info(),
                authority: ctx.accounts.fraction_authority.to_account_info(),
            },
        );
        burn(cpi_burn, required_fractions)?;

        // 2) NFT を再ミント
        let cpi_mint = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint:      ctx.accounts.target_mint.to_account_info(),       // owner チェックなし
                to:        ctx.accounts.target_nft_account.to_account_info(),
                authority: ctx.accounts.nft_mint_authority.to_account_info(),
            },
        );
        mint_to(cpi_mint, 1)?;

        msg!(
            "Merged {} fraction tokens into NFT on {}",
            required_fractions,
            ctx.accounts.target_mint.key(),
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SplitNft<'info> {
    /// CHECK: 元 NFT のトークンアカウント所有者検証をしていない
    #[account(mut)]
    pub source_nft_account:     AccountInfo<'info>,
    /// CHECK: 元 NFT のミントアカウント所有者検証をしていない
    #[account(mut)]
    pub source_spl_mint:        AccountInfo<'info>,

    /// CHECK: 分割用トークンのミント所有者検証をしていない
    #[account(mut)]
    pub fraction_mint:          AccountInfo<'info>,
    /// CHECK: 分割トークンを受け取るアカウント所有者検証をしていない
    #[account(mut)]
    pub fraction_account:       AccountInfo<'info>,

    /// トークン CPI 用
    pub token_program:          Program<'info, Token>,

    /// 分割操作を行うユーザー（署名のみ検証）
    pub user:                   Signer<'info>,
    /// 分割トークンミント権限を持つ署名者（署名のみ検証）
    pub fraction_authority:     Signer<'info>,
}

#[derive(Accounts)]
pub struct MergeNft<'info> {
    /// CHECK: 分割トークンのミント所有者検証をしていない
    #[account(mut)]
    pub fraction_mint:          AccountInfo<'info>,
    /// CHECK: 分割トークンをバーンするアカウント所有者検証をしていない
    #[account(mut)]
    pub fraction_account:       AccountInfo<'info>,

    /// CHECK: 再ミント先 NFT ミント所有者検証をしていない
    #[account(mut)]
    pub target_mint:            AccountInfo<'info>,
    /// CHECK: NFT を受け取るトークンアカウント所有者検証をしていない
    #[account(mut)]
    pub target_nft_account:     AccountInfo<'info>,

    pub token_program:          Program<'info, Token>,

    /// 分割トークンバーンおよびNFTミント権限を持つ署名者（署名のみ検証）
    pub fraction_authority:     Signer<'info>,
    pub nft_mint_authority:     Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("CPI 操作に失敗しました")]
    OperationFailed,
}
