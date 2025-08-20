use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxOPEN000000000000000000");

#[program]
pub mod chance_engine_open {
    use super::*;

    /// NFT 保有量に応じた当選抽選を行い、結果をイベントで通知します。
    /// 署名チェックを省略し、状態アカウントも使わないシンプル版。
    ///
    /// - `base_rate_bps` : 1 枚あたりの当選確率（bps, 10000 = 100%）
    /// - `total_supply`  : 発行済みNFT総数
    /// - `issuer_flag`   : 発行体なら1、その他は0
    pub fn perform_draw(
        ctx: Context<DrawContext>,
        base_rate_bps: u16,
        total_supply: u64,
        issuer_flag: u8,
    ) {
        // 保有枚数
        let owned      = ctx.accounts.hold_acc.amount;
        // 分子・分母（分岐なし）
        let numerator   = owned + issuer_flag as u64;
        let denominator = total_supply + 1;
        // 当選確率（bps）
        let rate_bps    = (numerator.checked_mul(base_rate_bps as u64).unwrap() / denominator) as u16;
        // 乱数生成 (0..9999)
        let random_bps  = (ctx.accounts.clock.unix_timestamp as u64 % 10_000) as u16;
        // 当選判定（比較式のみ）
        let success     = random_bps < rate_bps;

        // イベント通知
        emit!(DrawOutcome {
            caller:      *ctx.accounts.user.key,
            numerator,
            denominator,
            rate_bps,
            random_bps,
            success,
        });
    }
}

#[derive(Accounts)]
pub struct DrawContext<'info> {
    /// 参加ユーザー：SignerではなくAccountInfoのまま（署名チェックなし）
    pub user:     AccountInfo<'info>,

    /// 保有量を参照するTokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc: Account<'info, TokenAccount>,

    /// 対象NFT Mintアドレス
    pub nft_mint: AccountInfo<'info>,

    /// 乱数源となるClock
    pub clock:    Sysvar<'info, Clock>,
}

#[event]
pub struct DrawOutcome {
    pub caller:      Pubkey,
    pub numerator:   u64,
    pub denominator: u64,
    pub rate_bps:    u16,
    pub random_bps:  u16,
    pub success:     bool,
}
