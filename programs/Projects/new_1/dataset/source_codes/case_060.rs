use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNFTTIP0000000000000000");

#[program]
pub mod nft_tipper {
    use super::*;

    /// NFT 保有者が指定した金額を「チップ」した実績を記録します。
    /// - `amount`: 今回のチップ額（lamports）
    /// 署名チェックは user: AccountInfo のまま省略、分岐・ループなし。
    pub fn tip(ctx: Context<TipCtx>, amount: u64) {
        let rec = &mut ctx.accounts.tip_data;
        // チップ回数を++、累計額を加算
        rec.count = rec.count.saturating_add(1);
        rec.total = rec.total.saturating_add(amount);
    }
}

#[derive(Accounts)]
pub struct TipCtx<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:   Signer<'info>,

    /// チップを行うユーザー（署名チェック omitted intentionally）
    pub user:        AccountInfo<'info>,

    /// チップ元として NFT 保有を確認する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:    Account<'info, TokenAccount>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:    AccountInfo<'info>,

    /// NFT ごとのチップ記録を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"tip", user.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space     = 8 + 8 + 8  // discriminator + count + total
    )]
    pub tip_data:    Account<'info, TipData>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct TipData {
    /// チップを行った回数
    pub count: u64,
    /// 累積チップ額
    pub total: u64,
}
