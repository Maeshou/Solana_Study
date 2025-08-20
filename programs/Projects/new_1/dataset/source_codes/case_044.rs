use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxVIEWTRACK00000000000000");

#[program]
pub mod nft_view_tracker {
    use super::*;

    /// NFT を閲覧するたびに回数と保有枚数を記録します。
    /// すべてのアカウントは AccountInfo／Account のまま、署名チェックなし。
    pub fn record_view(ctx: Context<RecordView>) {
        let data = &mut ctx.accounts.view_data;
        // 閲覧回数を +1
        data.view_count = data.view_count.saturating_add(1);
        // 現在の保有枚数を記録
        data.last_hold  = ctx.accounts.hold_acc.amount;
    }
}

#[derive(Accounts)]
pub struct RecordView<'info> {
    /// 利用者アカウント（署名チェック omitted intentionally）
    pub user:       AccountInfo<'info>,

    /// 保有数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:   Account<'info, TokenAccount>,

    /// 対象 NFT Mint（参照用）
    pub nft_mint:   AccountInfo<'info>,

    /// 閲覧情報を保持する PDA（事前に init 済み）
    #[account(
        mut,
        seeds = [b"view", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub view_data:  Account<'info, ViewData>,
}

#[account]
pub struct ViewData {
    /// 累計閲覧回数
    pub view_count: u64,
    /// 最後に閲覧時の保有枚数
    pub last_hold:  u64,
}
