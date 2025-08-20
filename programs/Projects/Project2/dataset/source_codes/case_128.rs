use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4rove6YjFgDhqyQ5RBwzVL");

#[program]
pub mod premium_feature {
    use super::*;

    /// ロガーアカウントを初期化：所有者と初回ログをセット
    pub fn initialize_logger(ctx: Context<InitializeLogger>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        logger.owner            = ctx.accounts.authority.key();
        logger.access_count     = 0;
        logger.last_access_ts   = 0;
        Ok(())
    }

    /// プレミアム機能利用：  
    /// - `ticket`（データレス・SystemProgram所有・署名あり）で権限チェック  
    /// - 利用回数をカウント＋最終アクセス時刻を記録  
    /// - ここに実際のプレミアムロジックを実装
    pub fn use_premium(ctx: Context<UsePremium>) -> Result<()> {
        let logger = &mut ctx.accounts.logger;
        let now    = ctx.accounts.clock.unix_timestamp;

        // ログ更新
        logger.access_count   = logger.access_count.wrapping_add(1);
        logger.last_access_ts = now;

        // ダミーのプレミアム処理
        msg!("✨ Premium feature used by {}, total uses: {}", 
             ctx.accounts.authority.key(), logger.access_count);

        Ok(())
    }
}

#[account]
pub struct PremiumLogger {
    pub owner:          Pubkey, // ロガー所有者
    pub access_count:   u64,    // プレミアム機能利用回数
    pub last_access_ts: i64,    // 最終利用時刻
}

#[derive(Accounts)]
pub struct InitializeLogger<'info> {
    /// 新規ロガーアカウント（ランダムキーで init、PDA は使わない）
    #[account(
        init,
        payer = authority,
        space = 8   // discriminator
              + 32  // owner
              + 8   // access_count
              + 8   // last_access_ts
    )]
    pub logger:    Account<'info, PremiumLogger>,

    /// ロガー所有者（署名必須）
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UsePremium<'info> {
    /// 既存のロガー。has_one で owner フィールドと一致する signer を強制
    #[account(
        mut,
        has_one = authority
    )]
    pub logger:    Account<'info, PremiumLogger>,

    /// データレス「プレミアムチケット」アカウント  
    /// - System Program 所有  
    /// - かつそのキーの署名あり  
    #[account(
        constraint = ticket.to_account_info().owner == &System::id(),
        signer
    )]
    pub ticket:    UncheckedAccount<'info>,

    /// プレミアム権限を持つユーザー（logger.owner）  
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    /// 時刻取得用
    pub clock:     Sysvar<'info, Clock>,
}
