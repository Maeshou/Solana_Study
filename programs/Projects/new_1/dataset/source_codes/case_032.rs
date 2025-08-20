use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgjfLOCKVULN");

#[program]
pub mod account_locker_vuln {
    use super::*;

    /// 不正なアカウントをロックします。
    /// ────────────────────────────────────────────────
    /// ※ manager フィールドに対する署名チェックを敢えて置かず、
    ///    権限検証漏れのまま動作するため脆弱性が残ります。
    pub fn lock_user(ctx: Context<LockUser>) -> Result<()> {
        // ステータスをロック状態に切り替え
        let status = &mut ctx.accounts.user_status;
        status.locked = true;

        // ログ出力（誰が誰をロックしたか）
        msg!(
            "VULN: User {} has been locked by {}",
            status.user,
            ctx.accounts.manager.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LockUser<'info> {
    /// ロック対象ユーザーステータス
    /// - mut で更新許可
    /// - has_one = manager で事前に紐付けされていることのみ保証
    #[account(mut, has_one = manager)]
    pub user_status: Account<'info, UserStatus>,

    /// 管理者アカウント※署名チェックが抜けている（脆弱性）
    pub manager: AccountInfo<'info>,
}

#[account]
pub struct UserStatus {
    /// 対象ユーザーの Pubkey
    pub user: Pubkey,
    /// 管理者の Pubkey
    pub manager: Pubkey,
    /// ロックフラグ
    pub locked: bool,
}
