use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTFRACTION000000000000");

#[program]
pub mod nft_fractionalizer {
    use super::*;

    /// NFT を複数のフラクションに分割する準備を行います。
    /// - `total_pieces`: 分割後の総フラクション数  
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックはありません。
    pub fn start_fraction(ctx: Context<StartFraction>, total_pieces: u64) {
        let info = &mut ctx.accounts.fraction_data;
        info.owner          = *ctx.accounts.user.key;
        info.nft_mint       = ctx.accounts.nft_mint.key();
        info.total_pieces   = total_pieces;
        info.pieces_sold    = 0;
        info.is_fractional  = true;
    }

    /// 一部フラクションを販売し、販売数を累積します。
    /// - `sell_count`: 今回販売するフラクション数
    pub fn sell_fraction(ctx: Context<SellFraction>, sell_count: u64) {
        let info = &mut ctx.accounts.fraction_data;
        info.pieces_sold = info.pieces_sold.checked_add(sell_count).unwrap();
    }
}

#[derive(Accounts)]
pub struct StartFraction<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:     Signer<'info>,

    /// NFT 所有者（署名チェック omitted intentionally）
    pub user:          AccountInfo<'info>,

    /// 分割対象の NFT TokenAccount（所有者チェックのみ）
    #[account(
        constraint = nft_acc.owner == *user.key,
        constraint = nft_acc.mint  == nft_mint.key()
    )]
    pub nft_acc:       Account<'info, TokenAccount>,

    /// 対象 NFT Mint
    pub nft_mint:      AccountInfo<'info>,

    /// フラクション情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"fraction", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 32 + 32 + 8 + 8 + 1
    )]
    pub fraction_data: Account<'info, FractionData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SellFraction<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:     Signer<'info>,

    /// 購入者（署名チェック omitted intentionally）
    pub user:          AccountInfo<'info>,

    /// 既存のフラクション情報 PDA
    #[account(
        mut,
        seeds     = [b"fraction", owner.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub fraction_data: Account<'info, FractionData>,

    /// オリジナルの所有者検証用
    pub owner:         AccountInfo<'info>,

    /// 対象 NFT Mint
    pub nft_mint:      AccountInfo<'info>,
}

#[account]
pub struct FractionData {
    /// NFT 出品者の Pubkey
    pub owner:          Pubkey,
    /// 対象 NFT の Mint
    pub nft_mint:       Pubkey,
    /// 分割後の総フラクション数
    pub total_pieces:   u64,
    /// これまでに販売されたフラクション数
    pub pieces_sold:    u64,
    /// 分割済みフラグ
    pub is_fractional:  bool,
}
