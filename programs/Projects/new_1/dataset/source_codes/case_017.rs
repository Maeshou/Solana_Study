use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWxNEWSTATE9X3FT7Y8Z0HA1BC");

#[program]
pub mod nft_enhance_state {
    use super::*;

    pub fn enhance(ctx: Context<Enhance2>) -> Result<()> {
        // 擬似乱数生成（Clock から秒数を取得）
        let ts    = ctx.accounts.clock.unix_timestamp as u64;
        let rand  = ts
            .wrapping_mul(6364136223846793005)  // LCG 定数
            .wrapping_add(1)
            .wrapping_shr(32)                   // 大きな整数から上位 8 ビットを抽出
            % 100;

        // NFT トークンアカウントをクローズしてユーザーに lamports を戻す
        // close 属性で自動的に実行されるため、ここには明示的な CPI 呼び出しはなし
        // ──────────────────────────────────────────────
        // ctx.accounts.nft1_account.close(ctx.accounts.user.to_account_info())?;
        // ctx.accounts.nft2_account.close(ctx.accounts.user.to_account_info())?;
        // ... 以下同様に５口座

        // 合成結果をカスタム状態に書き込む
        let result = &mut ctx.accounts.result;
        result.user            = *ctx.accounts.user.key;
        result.timestamp       = ts;
        result.random_value    = rand as u8;
        result.participant_mints = [
            ctx.accounts.nft1_mint.key(),
            ctx.accounts.nft2_mint.key(),
            ctx.accounts.nft3_mint.key(),
            ctx.accounts.nft4_mint.key(),
            ctx.accounts.nft5_mint.key(),
        ];
        // 確率に応じたランク番号（1:＋1ランク, 2:＋2ランク, 3:Rainbow, 4:Special）
        result.outcome_rank = match rand {
            0..=79   => 1,
            80..=89  => 2,
            90..=97  => 3,
            _        => 4,
        };

        // イベント通知
        emit!(EnhanceStateEvent {
            user:       *ctx.accounts.user.key,
            rank:       result.outcome_rank,
            random:     result.random_value,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Enhance2<'info> {
    /// 合成実行者（署名必須）
    #[account(mut)]
    pub user:            Signer<'info>,

    /// 合成に使う５つの NFT トークンアカウントを自動クローズ
    #[account(mut, close = user)]
    pub nft1_account:    Account<'info, TokenAccount>,
    #[account(mut, close = user)]
    pub nft2_account:    Account<'info, TokenAccount>,
    #[account(mut, close = user)]
    pub nft3_account:    Account<'info, TokenAccount>,
    #[account(mut, close = user)]
    pub nft4_account:    Account<'info, TokenAccount>,
    #[account(mut, close = user)]
    pub nft5_account:    Account<'info, TokenAccount>,

    /// 元の NFT Mint アドレス
    pub nft1_mint:       Account<'info, Mint>,
    pub nft2_mint:       Account<'info, Mint>,
    pub nft3_mint:       Account<'info, Mint>,
    pub nft4_mint:       Account<'info, Mint>,
    pub nft5_mint:       Account<'info, Mint>,

    /// 合成結果を記録するカスタム状態アカウント
    #[account(init, payer = user, space = 8 + 32 + 8 + 1 + (5*32) + 1)]
    pub result:          Account<'info, EnhanceResult>,

    pub system_program:  Program<'info, System>,
    pub rent:            Sysvar<'info, Rent>,
    pub clock:           Sysvar<'info, Clock>,
}

#[account]
pub struct EnhanceResult {
    pub user:             Pubkey,
    pub timestamp:        u64,
    pub random_value:     u8,
    pub participant_mints: [Pubkey; 5],
    pub outcome_rank:     u8,
}

#[event]
pub struct EnhanceStateEvent {
    pub user:    Pubkey,
    pub rank:    u8,
    pub random:  u8,
}
