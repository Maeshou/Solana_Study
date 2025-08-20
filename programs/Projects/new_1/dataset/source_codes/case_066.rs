use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNOIFLOOP000000000000");

#[program]
pub mod reward_odds {
    use super::*;

    /// 発行者がミントしたときは `issuer_flag = 1`、そうでなければ `0` を渡してください。
    /// - `base_rate`   : 1 枚あたりの当選率 (0–100)
    /// - `total_mints` : これまでにミントされた総数
    /// - `issuer_flag`: 発行者なら 1、その他は 0
    ///
    /// 分岐・ループなしで分母・分子を調整し、抽選結果を記録します。
    pub fn compute_odds(
        ctx: Context<OddsCtx>,
        base_rate: u8,
        total_mints: u64,
        issuer_flag: u8,
    ) {
        // ① 保有枚数
        let held = ctx.accounts.hold_acc.amount;

        // ② 分母・分子を調整（分岐なし）
        let num = held + issuer_flag as u64;
        let den = total_mints + 1;

        // ③ 当選率 (0–100%) を計算
        let rate = (num
            .checked_mul(base_rate as u64).unwrap_or(0)
            .checked_div(den).unwrap_or(0))
            .min(100) as u8;

        // ④ 乱数 (0–99) を生成
        let rnd = (ctx.accounts.clock.unix_timestamp as u64 % 100) as u8;

        // ⑤ 当選フラグ（比較式のみ）
        let success = rnd < rate;

        // ⑥ PDA に書き込む
        let o = &mut ctx.accounts.outcome;
        o.numerator   = num;
        o.denominator = den;
        o.chance      = rate;
        o.random      = rnd;
        o.is_win      = success;
    }
}

#[derive(Accounts)]
pub struct OddsCtx<'info> {
    /// 参加ユーザー（署名チェックなし）
    pub user:       AccountInfo<'info>,

    /// 保有枚数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:   Account<'info, TokenAccount>,

    /// 対象 NFT の Mint（参照用）
    pub nft_mint:   AccountInfo<'info>,

    /// 抽選結果を記録する既存 PDA（事前に初期化必須）
    #[account(mut)]
    pub outcome:    Account<'info, OddsData>,

    /// 乱数源
    pub clock:      Sysvar<'info, Clock>,
}

#[account]
pub struct OddsData {
    /// 分子 (held + issuer_flag)
    pub numerator:   u64,
    /// 分母 (total_mints + 1)
    pub denominator: u64,
    /// 計算された当選率 (0–100)
    pub chance:      u8,
    /// 生成された乱数 (0–99)
    pub random:      u8,
    /// 当選フラグ
    pub is_win:      bool,
}
