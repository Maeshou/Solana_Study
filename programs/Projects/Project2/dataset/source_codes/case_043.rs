
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CancelSubscriptionCtxernt<'info> {
    #[account(mut)] pub subscription: Account<'info, DataAccount>,
    #[account(mut)] pub subscriber: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_043 {
    use super::*;

    pub fn cancel_subscription(ctx: Context<CancelSubscriptionCtxernt>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.subscription;
        // custom logic for cancel_subscription
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed cancel_subscription logic");
        Ok(())
    }
}
