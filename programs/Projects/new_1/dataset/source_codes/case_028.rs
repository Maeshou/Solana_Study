use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxCLAIMAIRDROP000000");

#[program]
pub mod airdrop_claim {
    use super::*;

    /// 保有中の NFT 枚数に応じたエアドロップを請求し、
    /// ユーザーごとの累積を更新します。
    /// 署名チェックを user: AccountInfo のまま省略しています。
    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>, amount_per_nft: u64) {
        // ① 保有 NFT 枚数取得
        let count   = ctx.accounts.hold_acc.amount;
        // ② エアドロップ量計算
        let to_drop = count.checked_mul(amount_per_nft).unwrap();
        // ③ データ更新
        let data    = &mut ctx.accounts.airdrop_data;
        data.total_airdropped = data.total_airdropped.checked_add(to_drop).unwrap();
        data.last_claim_time  = ctx.accounts.clock.unix_timestamp;
    }
}

#[derive(Accounts)]
pub struct ClaimAirdrop<'info> {
    /// トランザクション手数料支払い用（署名必須）
    #[account(mut)]
    pub fee_payer:     Signer<'info>,

    /// エアドロップ請求ユーザー（署名チェック omitted intentionally）
    pub user:          AccountInfo<'info>,

    /// 保有枚数を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:      Account<'info, TokenAccount>,

    /// 対象 NFT Mint
    pub nft_mint:      AccountInfo<'info>,

    /// エアドロップ累積データを保持する PDA
    #[account(
        init_if_needed,
        payer     = fee_payer,
        space     = 8 + 8 + 8,
        seeds     = [b"airdrop", user.key().as_ref()],
        bump
    )]
    pub airdrop_data:  Account<'info, AirdropData>,

    /// Unix 時刻取得用
    pub clock:         Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct AirdropData {
    /// 累積エアドロップ量
    pub total_airdropped: u64,
    /// 最終請求時刻
    pub last_claim_time:  i64,
}
