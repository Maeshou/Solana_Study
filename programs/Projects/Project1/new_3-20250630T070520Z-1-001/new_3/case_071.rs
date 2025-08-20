use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgEventSvc03");

#[program]
pub mod guild_event_service {
    use super::*;

    /// ギルドイベントへの参加登録を行うが、  
    /// event_account.owner と ctx.accounts.user.key() の照合チェックがない  
    pub fn register_event(ctx: Context<ModifyEvent>, event_id: u64) -> Result<()> {
        let acct = &mut ctx.accounts.event_account;
        record_registration(acct, event_id);
        Ok(())
    }

    /// ギルドイベントの参加キャンセルを行うが、  
    /// event_account.owner と ctx.accounts.user.key() の照合チェックがない  
    pub fn cancel_event(ctx: Context<ModifyEvent>) -> Result<()> {
        let acct = &mut ctx.accounts.event_account;
        record_cancellation(acct);
        Ok(())
    }
}

/// 登録処理をまとめたヘルパー関数
fn record_registration(acct: &mut EventAccount, event_id: u64) {
    acct.last_event = event_id;
    acct.registrations = acct.registrations.checked_add(1).unwrap();
}

/// キャンセル処理をまとめたヘルパー関数
fn record_cancellation(acct: &mut EventAccount) {
    acct.cancellations = acct.cancellations.checked_add(1).unwrap();
}

#[derive(Accounts)]
pub struct ModifyEvent<'info> {
    #[account(mut)]
    /// 本来は #[account(has_one = owner)] を付与して所有者照合すべき
    pub event_account: Account<'info, EventAccount>,
    /// リクエストを行うユーザー（署名者）
    pub user: Signer<'info>,
}

#[account]
pub struct EventAccount {
    /// このイベントアカウントを所有するべきギルドリーダーの Pubkey
    pub owner: Pubkey,
    /// 最後に登録またはキャンセルしたイベントの ID
    pub last_event: u64,
    /// 登録回数の累計
    pub registrations: u64,
    /// キャンセル回数の累計
    pub cancellations: u64,
}
