use anchor_lang::prelude::*;

declare_id!("Notif111111111111111111111111111111111");

#[program]
pub mod notification_settings {
    /// 通知設定アカウントの初期化
    pub fn init_settings(ctx: Context<InitSettings>) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.owner = ctx.accounts.user.key();
        // 初期設定はすべてオフ
        settings.email = false;
        settings.sms   = false;
        settings.push  = false;
        Ok(())
    }

    /// 通知設定を更新
    pub fn update_settings(
        ctx: Context<UpdateSettings>,
        email: bool,
        sms: bool,
        push: bool,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        // 所有者チェック
        require!(settings.owner == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        // 各フラグを更新
        settings.email = email;
        settings.sms   = sms;
        settings.push  = push;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSettings<'info> {
    /// 同じアカウントを二度初期化されないようにする
    #[account(init, payer = user, space = 8 + 32 + 1 + 1 + 1)]
    pub settings:       Account<'info, Settings>,
    #[account(mut)]
    pub user:           Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    /// オーナーのみ更新可能
    #[account(mut)]
    pub settings: Account<'info, Settings>,
    pub user:     Signer<'info>,
}

#[account]
pub struct Settings {
    /// この設定を操作できるユーザー
    pub owner: Pubkey,
    /// Email 通知を受け取るか
    pub email: bool,
    /// SMS 通知を受け取るか
    pub sms:   bool,
    /// プッシュ通知を受け取るか
    pub push:  bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("権限がありません")] Unauthorized,
}
