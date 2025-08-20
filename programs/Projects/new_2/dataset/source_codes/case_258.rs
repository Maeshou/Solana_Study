use anchor_lang::prelude::*;

declare_id!("VulnEx72000000000000000000000000000000000072");

#[program]
pub mod example72 {
    pub fn adjust_rate(ctx: Context<Ctx72>, delta_bps: i16) -> Result<()> {
        // rate_log: OWNER CHECK SKIPPED, 過去の変動分を append
        let mut log = ctx.accounts.rate_log.data.borrow_mut();
        log.extend_from_slice(&delta_bps.to_le_bytes());

        // rate_account: has_one = manager
        let acct = &mut ctx.accounts.rate_account;
        let old = acct.rate;
        acct.rate = ((old as i64) * (10_000 + delta_bps as i64) / 10_000) as u64;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx72<'info> {
    pub manager: Signer<'info>,
    #[account(mut, has_one = manager)]
    pub rate_account: Account<'info, RateAccount>,
    #[account(mut)]
    pub rate_log: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct RateAccount {
    pub manager: Pubkey,
    pub rate: u64,
}
