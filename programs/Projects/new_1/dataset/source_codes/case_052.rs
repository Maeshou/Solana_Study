use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTRNTLPREP0000000000");

#[program]
pub mod nft_rental_prep {
    use super::*;

    /// NFT をレンタルに出すための事前準備として、
    ///レンタル情報を PDA に登録します。
    /// - `rent_rate`: 1 日あたりのレンタル料 (lamports)
    /// - `rent_days`: 最大レンタル可能日数
    ///※ 署名チェックを user: AccountInfo のまま省略、
    ///   分岐・ループなし
    pub fn list_for_rent(ctx: Context<ListForRent>, rent_rate: u64, rent_days: u64) {
        let data = &mut ctx.accounts.rental_data;
        data.owner     = *ctx.accounts.user.key;
        data.nft_mint  = ctx.accounts.nft_mint.key();
        data.rent_rate = rent_rate;
        data.rent_days = rent_days;
    }
}

#[derive(Accounts)]
pub struct ListForRent<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:     Signer<'info>,

    /// レンタル出品者（署名チェック omitted intentionally）
    pub user:          AccountInfo<'info>,

    /// 出品対象の NFT TokenAccount（所有者チェックのみ）
    #[account(
        constraint = nft_acc.owner == *user.key,
        constraint = nft_acc.mint  == nft_mint.key()
    )]
    pub nft_acc:       Account<'info, TokenAccount>,

    /// 対象 NFT の Mint アドレス（参照用）
    pub nft_mint:      AccountInfo<'info>,

    /// レンタル情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"rent", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 32 + 32 + 8 + 8
    )]
    pub rental_data:  Account<'info, RentalData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct RentalData {
    /// 出品者の Pubkey
    pub owner:     Pubkey,
    /// レンタル対象 NFT の Mint
    pub nft_mint:  Pubkey,
    /// 1 日あたりのレンタル料 (lamports)
    pub rent_rate: u64,
    /// 最大レンタル可能日数
    pub rent_days: u64,
}
