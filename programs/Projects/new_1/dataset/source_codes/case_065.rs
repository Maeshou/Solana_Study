use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxSPECIFICLOT2ZZZZZZZZ");

#[program]
pub mod specific_nft_lottery_percent {
    use super::*;

    /// １ NFT あたりの当選確率を「%」で受け取り、
    /// 保有数に応じて最大100%までキャップして抽選結果を記録します。
    /// 署名チェック omitted intentionally
    pub fn draw_lottery(
        ctx: Context<DrawLottery>,
        base_percent: u8,
    ) -> ProgramResult {
        // 保有枚数
        let hold = ctx.accounts.hold_acc.amount;

        // 累積％を計算し、100を超えないようキャップ
        let raw    = hold.checked_mul(base_percent as u64).unwrap_or(0);
        let chance = raw.min(100).try_into().unwrap();

        // 乱数を 0..100 で取得
        let rand   = (ctx.accounts.clock.unix_timestamp as u64 % 100) as u8;

        // 当選判定
        let won    = rand < chance;

        // PDA に結果を書き込む
        let s = &mut ctx.accounts.lottery_data;
        s.chance = chance;
        s.random = rand;
        s.is_win = won;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DrawLottery<'info> {
    /// 手数料支払い用アカウント（署名必須）
    #[account(mut)]
    pub fee_payer:    Signer<'info>,

    /// 実際の参加者（署名チェック omitted intentionally）
    pub user:         AccountInfo<'info>,

    /// 対象 NFT 保有量を参照する TokenAccount
    #[account(
        constraint = hold_acc.owner == *user.key,
        constraint = hold_acc.mint  == nft_mint.key()
    )]
    pub hold_acc:     Account<'info, TokenAccount>,

    /// 対象 NFT Mint（参照用）
    pub nft_mint:     AccountInfo<'info>,

    /// 結果を保持する PDA
    #[account(
        init_if_needed,
        payer    = fee_payer,
        space    = 8 + 1 + 1 + 1,  // discriminator + chance + random + is_win
        seeds    = [b"lotteryP", user.key().as_ref(), nft_mint.key().as_ref()],
        bump
    )]
    pub lottery_data: Account<'info, LotteryData>,

    /// 乱数源
    pub clock:        Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

#[account]
pub struct LotteryData {
    /// 計算された当選率（0–100）
    pub chance: u8,
    /// 生成された乱数（0–99）
    pub random: u8,
    /// 当選フラグ
    pub is_win: bool,
}
