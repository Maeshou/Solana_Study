
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncrementCtxqutr<'info> {
    #[account(mut)] pub counter: Account<'info, DataAccount>,
    #[account(mut)] pub caller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_060 {
    use super::*;

    pub fn increment(ctx: Context<IncrementCtxqutr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.counter;
        // custom logic for increment
        assert!(ctx.accounts.counter.data > 0); acct.data -= amount;
        msg!("Executed increment logic");
        Ok(())
    }
}
