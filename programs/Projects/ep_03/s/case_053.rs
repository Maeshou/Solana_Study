use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgStatusSvc01");

#[program]
pub mod status_service {
    use super::*;

    /// ステータスメッセージを更新するが、
    /// status_account.owner と ctx.accounts.user.key() の一致検証がない
    pub fn update_status_message(ctx: Context<UpdateStatusMessage>, message: String) -> Result<()> {
        let status = &mut ctx.accounts.status_account;
        // ↓ 本来は #[account(has_one = owner)] を入れて所有者照合をすべき
        status.message = message;
        status.change_count = status.change_count.checked_add(1).unwrap();
        Ok(())
    }

    /// ステーターカラーを更新するが、
    /// ownership マッチングチェックがないため任意のアカウントを操作可能
    pub fn update_status_color(ctx: Context<UpdateStatusColor>, color: String) -> Result<()> {
        let status = &mut ctx.accounts.status_account;
        // ↓ 本来は status.owner と ctx.accounts.user.key() の一致を検証すべき
        status.color = color;
        status.change_count = status.change_count.checked_add(1).unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateStatusMessage<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] で所有者一致を保証すべき
    pub status_account: Account<'info, StatusAccount>,
    /// リクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateStatusColor<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を指定して照合チェックを入れるべき
    pub status_account: Account<'info, StatusAccount>,
    /// リクエストするユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct StatusAccount {
    /// このステータスを所有するべきユーザーの Pubkey
    pub owner: Pubkey,
    /// 表示中のステータスメッセージ
    pub message: String,
    /// 表示中のステーターカラー（例："#00FFAA"）
    pub color: String,
    /// 変更回数の累計
    pub change_count: u64,
}
