use anchor_lang::prelude::*;

// ── アカウントデータはファイル冒頭に定義 ──
#[account]
#[derive(Default)]
pub struct TipJar {
    pub bump:         u8,      // PDA bump
    pub total_tips:   u64,     // 累計チップ金額
    pub top_tipper:   Pubkey,  // 現在の最高チッパー
    pub top_tip:      u64,     // 最高チップ額
    pub last_tip_ts:  i64,     // 最終チップ時刻
}

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzUQ");

#[program]
pub mod tip_jar {
    use super::*;

    /// TipJar 初期化：主要フィールドだけセットし、残りは Default
    pub fn initialize_tip_jar(
        ctx: Context<InitializeTipJar>,
    ) -> Result<()> {
        let jar  = &mut ctx.accounts.tip_jar;
        let bump = *ctx.bumps.get("tip_jar").unwrap();
        let now  = ctx.accounts.clock.unix_timestamp;

        *jar = TipJar {
            bump,
            last_tip_ts: now,
            ..Default::default()
        };
        Ok(())
    }

    /// チップ送信：累計を加算し、最高チップ更新ロジックを `if` で分離
    pub fn tip(
        ctx: Context<Tip>,
        amount: u64,
    ) -> Result<()> {
        let jar = &mut ctx.accounts.tip_jar;
        let now = ctx.accounts.clock.unix_timestamp;

        // 累計チップ金額を加算
        jar.total_tips = jar.total_tips.wrapping_add(amount);

        // 最高チップ額更新判定
        if amount > jar.top_tip {
            jar.top_tip     = amount;
            jar.top_tipper  = ctx.accounts.tipper.key();
        }

        // 最終チップ時刻を更新
        jar.last_tip_ts = now;
        Ok(())
    }

    /// リセット：累計・最高チップ情報をクリアし、時刻だけ更新
    pub fn reset(
        ctx: Context<ResetTipJar>,
    ) -> Result<()> {
        let jar  = &mut ctx.accounts.tip_jar;
        let bump = jar.bump;
        let now  = ctx.accounts.clock.unix_timestamp;

        // bump は保持しつつ、その他をデフォルトに戻す
        *jar = TipJar {
            bump,
            last_tip_ts: now,
            ..Default::default()
        };
        Ok(())
    }
}

// ── コンテキスト定義は末尾に ──
#[derive(Accounts)]
pub struct InitializeTipJar<'info> {
    #[account(
        init_zeroed,
        payer = authority,
        seeds = [b"tip_jar", authority.key().as_ref()],
        bump,
        space = 8   // discriminator
              +1   // bump
              +8   // total_tips
              +32  // top_tipper
              +8   // top_tip
              +8   // last_tip_ts
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// タイムスタンプ取得用
    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(
        mut,
        seeds = [b"tip_jar", authority.key().as_ref()],
        bump = tip_jar.bump,
    )]
    pub tip_jar: Account<'info, TipJar>,

    /// チッパー（署名必須）
    #[account(signer)]
    pub tipper: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetTipJar<'info> {
    #[account(
        mut,
        seeds = [b"tip_jar", authority.key().as_ref()],
        bump = tip_jar.bump,
    )]
    pub tip_jar: Account<'info, TipJar>,

    /// 管理者（署名必須）
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
