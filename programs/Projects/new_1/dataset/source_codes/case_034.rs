use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxGOVERNANCE000000000000");

#[program]
pub mod governance {
    use super::*;

    /// NFT 保有枚数に応じてグローバル投票数を更新します。
    /// - `weight_per_nft`: 1 枚あたりの投票ウェイト
    /// ※ 署名チェックを `user: AccountInfo<'info>` のまま省略
    pub fn vote(ctx: Context<VoteCtx>, weight_per_nft: u64) {
        // NFT 保有枚数取得
        let count = ctx.accounts.hold_acc.amount;
        // 付与されるウェイト計算
        let delta = count.checked_mul(weight_per_nft).unwrap();
        // グローバルデータ更新
        let data = &mut ctx.accounts.global_data;
        data.total_votes      = data.total_votes.checked_add(delta).unwrap();
        data.last_voter       = *ctx.accounts.user.key;
        data.last_vote_time   = ctx.accounts.clock.unix_timestamp;
    }
}

#[derive(Accounts)]
pub struct VoteCtx<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// 投票ユーザー（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// NFT 保有数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:     Account<'info, TokenAccount>,

    /// 対象 NFT の Mint アドレス（制約チェック用）
    pub nft_mint:     AccountInfo<'info>,

    /// グローバル投票データを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"global-votes"],
        bump,
        space     = 8 + 8 + 32 + 8
    )]
    pub global_data: Account<'info, GlobalData>,

    /// Unix 時刻取得用
    pub clock:        Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct GlobalData {
    /// 累積投票数
    pub total_votes:    u64,
    /// 最後に投票したユーザー
    pub last_voter:     Pubkey,
    /// 最後に投票した時刻
    pub last_vote_time: i64,
}
