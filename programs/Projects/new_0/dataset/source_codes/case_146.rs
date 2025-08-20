use anchor_lang::prelude::*;

declare_id!("ThrA111111111111111111111111111111111111");

#[program]
pub mod threshold_alert {
    /// アラート設定アカウントの初期化
    pub fn init_alert(ctx: Context<InitAlert>, threshold: u64) -> Result<()> {
        // しきい値は1以上である必要がある
        require!(threshold > 0, ErrorCode::InvalidThreshold);

        let alert = &mut ctx.accounts.alert;
        alert.owner = ctx.accounts.user.key();
        alert.threshold = threshold;
        alert.active = true;
        Ok(())
    }

    /// しきい値の更新
    pub fn update_alert(ctx: Context<UpdateAlert>, threshold: u64) -> Result<()> {
        // しきい値のバリデーション
        require!(threshold > 0, ErrorCode::InvalidThreshold);
        let alert = &mut ctx.accounts.alert;
        // 所有者のみ更新可能
        require!(alert.owner == ctx.accounts.user.key(), ErrorCode::AccessDenied);

        alert.threshold = threshold;
        Ok(())
    }

    /// アラートの無効化
    pub fn disable_alert(ctx: Context<DisableAlert>) -> Result<()> {
        let alert = &mut ctx.accounts.alert;
        // 所有者のみ操作可能
        require!(alert.owner == ctx.accounts.user.key(), ErrorCode::AccessDenied);

        alert.active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAlert<'info> {
    /// 再初期化を防ぐ
    #[account(init, payer = user, space = 8 + 32 + 8 + 1)]
    pub alert:          Account<'info, AlertAccount>,
    #[account(mut)]
    pub user:           Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAlert<'info> {
    /// Account<> 型でオーナーチェック
    #[account(mut)]
    pub alert: Account<'info, AlertAccount>,
    pub user:  Signer<'info>,
}

#[derive(Accounts)]
pub struct DisableAlert<'info> {
    #[account(mut)]
    pub alert: Account<'info, AlertAccount>,
    pub user:  Signer<'info>,
}

#[account]
pub struct AlertAccount {
    /// このアラートを操作できるユーザー
    pub owner:     Pubkey,
    /// アラート条件となるしきい値
    pub threshold: u64,
    /// 有効フラグ
    pub active:    bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アクセス権限がありません")] AccessDenied,
    #[msg("しきい値は1以上である必要があります")] InvalidThreshold,
}
