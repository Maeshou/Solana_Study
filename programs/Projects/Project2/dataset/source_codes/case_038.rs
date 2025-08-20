use anchor_lang::prelude::*;

declare_id!("ZER0c0py1111111111111111111111111111111111");

#[program]
pub mod zero_copy_example {
    use super::*;
    pub fn init_large_account(ctx: Context<InitLargeAccount>) -> Result<()> {
        let mut event_log = ctx.accounts.event_log.load_init()?;
        event_log.admin = *ctx.accounts.admin.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLargeAccount<'info> {
    // zero-copyアカウントを初期化。オーナーはカレントプログラムになる。
    #[account(zero)]
    pub event_log: AccountLoader<'info, EventLog>,
    pub admin: Signer<'info>,
}

#[account(zero_copy)]
pub struct EventLog {
    pub admin: Pubkey,
    pub events: [u8; 1024 * 8], // 8KBのデータ
}