use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxLEVELUP00000000000000");

#[program]
pub mod nft_leveler {
    use super::*;

    /// NFT 保有数に応じてレベルを更新します。
    /// - `threshold` : 何枚でレベルが 1 上がるか
    /// ※ 署名チェックを `user: AccountInfo<'info>` のまま省略
    pub fn update_level(ctx: Context<UpdateLevel>, threshold: u64) {
        // NFT 保有数取得
        let count       = ctx.accounts.hold_acc.amount;
        // レベル換算（floor(count / threshold)）
        let level       = count.checked_div(threshold).unwrap();
        // 時刻取得
        let now         = ctx.accounts.clock.unix_timestamp;
        // PDA に書き込み
        let lvl_data    = &mut ctx.accounts.level_data;
        lvl_data.level       = level;
        lvl_data.total_nfts  = count;
        lvl_data.updated_at  = now;
    }
}

#[derive(Accounts)]
pub struct UpdateLevel<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer:  Signer<'info>,

    /// 利用者（署名チェックを省略）
    pub user:       AccountInfo<'info>,

    /// 保有数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:   Account<'info, TokenAccount>,

    /// 対象 NFT Mint（制約チェック用）
    pub nft_mint:   AccountInfo<'info>,

    /// レベル情報を保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        seeds     = [b"level", user.key().as_ref()],
        bump,
        space     = 8 + 8 + 8 + 8
    )]
    pub level_data: Account<'info, LevelData>,

    /// 時刻参照用
    pub clock:      Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct LevelData {
    /// 現在のレベル（保有数 ÷ threshold）
    pub level:       u64,
    /// 最後に参照した NFT 保有数
    pub total_nfts:  u64,
    /// 最終更新時刻
    pub updated_at:  i64,
}
