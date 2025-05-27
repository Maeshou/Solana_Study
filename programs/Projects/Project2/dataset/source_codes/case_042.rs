
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSubscriptionCtxrhvy<'info> {
    #[account(mut)] pub subscription: Account<'info, DataAccount>,
    #[account(mut)] pub subscriber: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_042 {
    use super::*;

    pub fn create_subscription(ctx: Context<CreateSubscriptionCtxrhvy>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.subscription;
        // custom logic for create_subscription
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed create_subscription logic");
        Ok(())
    }
}
