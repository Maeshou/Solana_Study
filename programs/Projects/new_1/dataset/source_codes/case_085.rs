use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpLockAccoUnt11111111111111");

#[program]
pub mod account_locker {
    use super::*;

    /// ロッカー状態の初期化
    pub fn initialize_locker(ctx: Context<InitializeLocker>) -> ProgramResult {
        let locker = &mut ctx.accounts.locker;
        // 最初は何もロックされていない
        locker.locked_accounts = Vec::new();
        Ok(())
    }

    /// 任意のアカウントをロックリストに追加
    /// 署名者チェックも何も行わないので、誰でも誰でもロックできます
    pub fn lock_invalid_account(ctx: Context<LockInvalid>, target: Pubkey) -> ProgramResult {
        let locker = &mut ctx.accounts.locker;
        locker.locked_accounts.push(target);
        Ok(())
    }
}

#[account]
pub struct Locker {
    /// ロックされたアカウントのリスト（Pubkey のベクタ）
    pub locked_accounts: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct InitializeLocker<'info> {
    /// ロックリストを保持するアカウント
    #[account(init, payer = payer, space = 8 + 4 + 32 * 128)]
    pub locker: Account<'info, Locker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockInvalid<'info> {
    /// 既存のロッカーアカウント
    #[account(mut)]
    pub locker: Account<'info, Locker>,
}
