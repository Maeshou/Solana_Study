use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTRNTL00000000000000");

#[program]
pub mod nft_rental {
    use super::*;

    /// 事前準備：NFT をレンタルに出品します。
    /// - `rent_rate`: 1日あたりのレンタル料 (lamports)
    /// - `rent_days`: 最大レンタル可能日数
    pub fn list_for_rent(ctx: Context<ListForRent>, rent_rate: u64, rent_days: u64) {
        let info = &mut ctx.accounts.rental_data;
        info.owner     = *ctx.accounts.owner.key;
        info.nft_mint  = ctx.accounts.nft_mint.key();
        info.rent_rate = rent_rate;
        info.rent_days = rent_days;
        info.paid_fee  = 0;
        info.renter    = Pubkey::default();
        info.is_active = false;
    }

    /// 貸し出し実行：指定日数分の料金を支払い、レンタル状態にします。
    /// - `days`: 実際にレンタルする日数
    pub fn rent_nft(ctx: Context<RentNft>, days: u64) {
        let data = &mut ctx.accounts.rental_data;
        // 支払い額を計算して累積
        let fee = data.rent_rate.checked_mul(days).unwrap();
        data.paid_fee = data.paid_fee.saturating_add(fee);
        // レンタル状態に更新
        data.renter    = *ctx.accounts.renter.key;
        data.is_active = true;
    }
}

#[derive(Accounts)]
pub struct ListForRent<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:     Signer<'info>,

    /// NFT の所有者（署名チェック omitted intentionally）
    pub owner:         AccountInfo<'info>,

    /// 出品対象の NFT TokenAccount（所有者チェックのみ）
    #[account(
        constraint = nft_acc.owner == *owner.key,
        constraint = nft_acc.mint  == nft_mint.key()
    )]
    pub nft_acc:       Account<'info, TokenAccount>,

    /// 対象 NFT Mint
    pub nft_mint:      AccountInfo<'info>,

    /// レンタル情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"rent", owner.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 32 + 32 + 8 + 8 + 8 + 32 + 1
    )]
    pub rental_data:  Account<'info, RentalData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RentNft<'info> {
    /// 手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:   Signer<'info>,

    /// 出品者（PDA シード用だが署名チェック省略）
    pub owner:       AccountInfo<'info>,

    /// レンタルを行う借り手（署名チェック omitted intentionally）
    pub renter:      AccountInfo<'info>,

    /// 既存の RentalData PDA
    #[account(
        mut,
        seeds     = [b"rent", owner.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub rental_data: Account<'info, RentalData>,

    /// 対象 NFT Mint
    pub nft_mint:    AccountInfo<'info>,
}

#[account]
pub struct RentalData {
    /// NFT 出品者の Pubkey
    pub owner:      Pubkey,
    /// レンタル対象 NFT の Mint
    pub nft_mint:   Pubkey,
    /// 1日あたりのレンタル料
    pub rent_rate:  u64,
    /// 最大レンタル日数
    pub rent_days:  u64,
    /// 累積支払い済み手数料
    pub paid_fee:   u64,
    /// 現在の借り手（Pubkey::default() なら未貸出）
    pub renter:     Pubkey,
    /// 貸出中フラグ
    pub is_active:  bool,
}
