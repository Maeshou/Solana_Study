use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxLOTTERY123456789ABCDEFG");

#[program]
pub mod nft_lottery {
    use super::*;

    /// 特定の NFT ミントの保有量に応じて当選確率を決定し、
    /// 抽選を実行して結果を記録します。
    ///
    /// # 引数
    /// - `base_bps`: 1 枚あたりの当選確率（basis points, 10000 = 100%）
    pub fn enter_lottery(
        ctx: Context<EnterLottery>,
        base_bps: u16,
    ) -> Result<()> {
        let clock = Clock::get()?;
        // 擬似乱数を 0..9999 の範囲で生成
        let rand_bps = (clock.unix_timestamp as u64 % 10_000) as u16;

        // ユーザーの保有量
        let hold = ctx.accounts.user_nft_account.amount;
        // トータル当選確率（basis points）
        let mut chance = hold
            .checked_mul(base_bps as u64)
            .unwrap()
            .min(10_000) as u16;

        // 結果判定
        let win = rand_bps < chance;

        // 結果を状態に保存
        let result = &mut ctx.accounts.result;
        result.user        = *ctx.accounts.user.key;
        result.hold_amount = hold;
        result.chance_bps  = chance;
        result.random_bps  = rand_bps;
        result.won         = win;

        // イベントで通知
        emit!(LotteryEvent {
            user:       *ctx.accounts.user.key,
            hold:       hold,
            chance_bps: chance,
            random_bps: rand_bps,
            won:        win,
        });

        Ok(())
    }
}

/// 入力アカウント定義
#[derive(Accounts)]
pub struct EnterLottery<'info> {
    /// 抽選に参加するユーザー（署名必須）
    #[account(mut)]
    pub user:             Signer<'info>,

    /// 指定の NFT ミントを保有する TokenAccount
    #[account(
        constraint = user_nft_account.owner == *user.key,
        constraint = user_nft_account.mint == nft_mint.key(),
    )]
    pub user_nft_account: Account<'info, TokenAccount>,

    /// 対象 NFT の Mint アドレス
    pub nft_mint:         AccountInfo<'info>,

    /// 結果を保存するアカウント（PDA）
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 2 + 2 + 1,
        seeds = [b"lottery", user.key().as_ref()],
        bump,
    )]
    pub result:           Account<'info, LotteryResult>,

    pub system_program:   Program<'info, System>,
    pub rent:             Sysvar<'info, Rent>,
}

/// 抽選結果を保持する状態
#[account]
pub struct LotteryResult {
    /// ユーザー
    pub user:        Pubkey,
    /// 保有枚数
    pub hold_amount: u64,
    /// トータル当選確率（bps）
    pub chance_bps:  u16,
    /// 生成された乱数（bps）
    pub random_bps:  u16,
    /// 当選フラグ
    pub won:         bool,
}

/// 抽選イベント
#[event]
pub struct LotteryEvent {
    pub user:       Pubkey,
    pub hold:       u64,
    pub chance_bps: u16,
    pub random_bps: u16,
    pub won:        bool,
}
