
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCounterCtxcyir<'info> {
    #[account(mut)] pub counter: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_059 {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounterCtxcyir>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.counter;
        // custom logic for initialize_counter
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed initialize_counter logic");
        Ok(())
    }
}
