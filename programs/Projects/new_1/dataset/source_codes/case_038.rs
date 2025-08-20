use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTCRAFT0000000000000");

#[program]
pub mod nft_crafter {
    use super::*;

    /// ２つの NFT を「クラフト」した実績をカウントします。
    /// - `left` と `right` の両方から同じ枚数だけ使用したとみなし、
    ///   使用枚数を累積します。
    /// ※ 署名チェックはすべて `AccountInfo` のまま省略
    pub fn craft(ctx: Context<CraftCtx>) {
        // 両方保有枚数のうち少ない方を使用数とみなす
        let left_amt  = ctx.accounts.left.amount;
        let right_amt = ctx.accounts.right.amount;
        let used      = left_amt.min(right_amt);

        // PDA の状態更新
        let rec = &mut ctx.accounts.craft_data;
        rec.times        = rec.times.saturating_add(1);
        rec.total_left   = rec.total_left.saturating_add(used);
        rec.total_right  = rec.total_right.saturating_add(used);
        rec.total_crafted = rec.total_crafted.saturating_add(used);
    }
}

#[derive(Accounts)]
pub struct CraftCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// クラフトを実行するユーザー（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 左側に使う NFT トークンアカウント
    #[account(mut, constraint = left.owner == *user.key)]
    pub left:         Account<'info, TokenAccount>,

    /// 右側に使う NFT トークンアカウント
    #[account(mut, constraint = right.owner == *user.key)]
    pub right:        Account<'info, TokenAccount>,

    /// クラフト実績を保持する PDA
    #[account(
        init_if_needed,
        payer = fee_payer,
        seeds = [b"craft", user.key().as_ref()],
        bump,
        space = 8  /* discriminator */
              + 8  /* times */
              + 8  /* total_left */
              + 8  /* total_right */
              + 8  /* total_crafted */
    )]
    pub craft_data:  Account<'info, CraftData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct CraftData {
    /// クラフトを実行した回数
    pub times:          u64,
    /// 左側トークンから使用した累計枚数
    pub total_left:     u64,
    /// 右側トークンから使用した累計枚数
    pub total_right:    u64,
    /// 合計クラフト枚数（used × times）
    pub total_crafted:  u64,
}
