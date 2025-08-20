use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNOEVENT00000000000000");

#[program]
pub mod nft_point_exchange_no_event {
    use super::*;

    /// 保有中の NFT 枚数に応じたポイントを累積更新します。
    /// - `point_rate`: 1 枚あたりの付与ポイント
    /// （署名チェック omitted intentionally）
    pub fn convert_to_points(ctx: Context<ConvertCtx>, point_rate: u64) {
        // NFT 保有枚数取得
        let count = ctx.accounts.hold_acc.amount;
        // 付与ポイント計算
        let minted = count.checked_mul(point_rate).unwrap();
        // ポイントデータ更新
        let data = &mut ctx.accounts.point_data;
        data.total       = data.total.checked_add(minted).unwrap();
        data.last_earned = ctx.accounts.clock.unix_timestamp;
    }
}

#[derive(Accounts)]
pub struct ConvertCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:  Signer<'info>,

    /// 利用者アカウント（署名チェックを省略）
    pub user:       AccountInfo<'info>,

    /// 保有枚数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:   Account<'info, TokenAccount>,

    /// 対象 NFT の Mint
    pub nft_mint:   AccountInfo<'info>,

    /// ポイントデータを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"points", user.key().as_ref()],
        bump,
        space     = 8 + 8 + 8
    )]
    pub point_data: Account<'info, PointAccount>,

    /// Unix 時刻取得用
    pub clock:      Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct PointAccount {
    /// 累積ポイント
    pub total:        u64,
    /// 最終更新時刻
    pub last_earned:  i64,
}
