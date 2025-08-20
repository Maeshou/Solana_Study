use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxPOINTEXCHG000000000000");

#[program]
pub mod nft_point_exchange {
    use super::*;

    /// NFT 保有数に応じてポイントを付与し、累積を更新します。
    /// - `point_rate`: 1 枚あたりのポイント付与量  
    /// ※ 署名チェックを省略し、分岐やループは使いません
    pub fn convert_to_points(ctx: Context<ConvertCtx>, point_rate: u64) {
        // NFT 保有数
        let count  = ctx.accounts.hold_acc.amount;
        // 付与ポイント計算
        let minted = count.checked_mul(point_rate).unwrap();
        // ポイントデータ更新
        let data   = &mut ctx.accounts.point_data;
        data.total        = data.total.checked_add(minted).unwrap();
        data.last_earned  = ctx.accounts.clock.unix_timestamp;
        // イベント通知
        emit!(ConvertEvent {
            user:   *ctx.accounts.user.key,
            minted,
            total:  data.total,
        });
    }
}

#[derive(Accounts)]
pub struct ConvertCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:      Signer<'info>,

    /// 利用者アカウント（署名チェック omitted intentionally）
    pub user:           AccountInfo<'info>,

    /// NFT 保有数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:       Account<'info, TokenAccount>,

    /// ポイントデータを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        space     = 8 + 8 + 8,
        seeds     = [b"points", user.key().as_ref()],
        bump
    )]
    pub point_data:     Account<'info, PointAccount>,

    /// NFT の Mint アドレス（制約チェック用）
    pub nft_mint:       AccountInfo<'info>,

    /// 乱数や時刻参照用
    pub clock:          Sysvar<'info, Clock>,

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

#[event]
pub struct ConvertEvent {
    pub user:   Pubkey,
    pub minted: u64,
    pub total:  u64,
}
