
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RenewSubscriptionCtxycka<'info> {
    #[account(mut)] pub subscription: Account<'info, DataAccount>,
    #[account(mut)] pub subscriber: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_044 {
    use super::*;

    pub fn renew_subscription(ctx: Context<RenewSubscriptionCtxycka>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.subscription;
        // custom logic for renew_subscription
        for _ in 0..amount { acct.data += 1; }
        msg!("Executed renew_subscription logic");
        Ok(())
    }
}
