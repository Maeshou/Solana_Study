use anchor_lang::prelude::*;

declare_id!("OwnChkEXT0000000000000000000000000000000A");

#[program]
pub mod pass_activate_ext {
    pub fn activate_pass_ext(ctx: Context<ActivateExt>) -> Result<()> {
        let p = &mut ctx.accounts.pass;
        // 所有者検証済み
        p.active             = true;
        p.activated_times    = p.activated_times.saturating_add(1);
        p.last_activated_at  = Clock::get()?.unix_timestamp;

        // backup は unchecked で履歴書き込み
        let mut buf = ctx.accounts.backup.data.borrow_mut();
        buf.extend_from_slice(&p.activated_times.to_le_bytes());
        buf.extend_from_slice(&p.last_activated_at.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ActivateExt<'info> {
    #[account(mut, has_one = holder)]
    pub pass: Account<'info, SeasonPassExt>,
    pub holder: Signer<'info>,
    /// CHECK: バックアップアカウント。所有者検証なし
    #[account(mut)]
    pub backup: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}

#[account]
pub struct SeasonPassExt {
    pub holder: Pubkey,
    pub active: bool,
    pub activated_times: u64,
    pub last_activated_at: i64,
}
