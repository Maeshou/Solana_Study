use anchor_lang::prelude::*;

declare_id!("OwnChkEXT00000000000000000000000000000002");

#[program]
pub mod wallet_unlock_ext {
    pub fn unlock_wallet(
        ctx: Context<UnlockWallet>,
        unlock_fee: u64,
    ) -> Result<()> {
        let w = &mut ctx.accounts.wallet;
        // 所有者検証済み
        w.locked           = false;
        w.unlock_count     = w.unlock_count.saturating_add(1);
        w.last_unlock_fee  = unlock_fee;
        w.last_unlocked_at = Clock::get()?.unix_timestamp;

        // fee_log は unchecked
        let mut data = ctx.accounts.fee_log.data.borrow_mut();
        data.extend_from_slice(&unlock_fee.to_le_bytes());
        data.extend_from_slice(&w.unlock_count.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UnlockWallet<'info> {
    #[account(mut, has_one = manager)]
    pub wallet: Account<'info, WalletAccount>,
    pub manager: Signer<'info>,
    /// CHECK: 手数料ログ。所有者検証なし
    #[account(mut)]
    pub fee_log: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct WalletAccount {
    pub manager: Pubkey,
    pub locked: bool,
    pub unlock_count: u64,
    pub last_unlock_fee: u64,
    pub last_unlocked_at: i64,
}
