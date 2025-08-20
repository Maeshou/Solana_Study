use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭にタプル構造体で定義 ──
#[account]
#[derive(Default)]
pub struct TimeCapsule(pub u8, pub Vec<(i64, Vec<u8>)>);

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod time_capsule {
    use super::*;

    /// TimeCapsule アカウント初期化
    pub fn initialize(
        ctx: Context<Initialize>,
    ) -> Result<()> {
        // init_zeroed によって tuple の Vec は空のまま
        // bump のみ設定
        let cap = &mut ctx.accounts.capsule;
        cap.0 = *ctx.bumps.get("capsule").unwrap();
        Ok(())
    }

    /// データ預け入れ：unlock_ts と生データを登録
    pub fn deposit(
        ctx: Context<ModifyCapsule>,
        unlock_ts: i64,
        data: Vec<u8>,
    ) -> Result<()> {
        // 期限を設定してそのまま push
        let cap = &mut ctx.accounts.capsule.1;
        cap.push((unlock_ts, data));
        Ok(())
    }

    /// 期限到来分の一括削除：now より古いエントリをフィルタ
    pub fn purge_expired(
        ctx: Context<ModifyCapsule>,
    ) -> Result<()> {
        let entries = &mut ctx.accounts.capsule.1;
        let now     = ctx.accounts.clock.unix_timestamp;
        // 条件は単一：期限内のものだけ残す
        entries.retain(|&(ts, _)| ts > now);
        Ok(())
    }

    /// 利用可能分のカウント報告：期限到来分を数えてログ出力
    pub fn count_ready(
        ctx: Context<ModifyCapsule>,
    ) -> Result<()> {
        let entries = &ctx.accounts.capsule.1;
        let now     = ctx.accounts.clock.unix_timestamp;
        let mut cnt = 0u64;
        for &(ts, _) in entries.iter() {
            if now >= ts {
                cnt = cnt.wrapping_add(1);
            }
        }
        msg!("Ready entries: {}", cnt);
        Ok(())
    }
}

// ── Context 定義は末尾に ──
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"capsule", authority.key().as_ref()],
        bump,
        // discriminator(8) + bump(1) + Vec<(i64, Vec<u8>)> (max10件: 10*(8+4+100))
        space = 8 + 1 + 4 + 10 * (8 + 4 + 100)
    )]
    pub capsule: Account<'info, TimeCapsule>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyCapsule<'info> {
    #[account(
        mut,
        seeds = [b"capsule", authority.key().as_ref()],
        bump = capsule.0,
    )]
    pub capsule: Account<'info, TimeCapsule>,

    /// 操作主体（署名必須）
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
