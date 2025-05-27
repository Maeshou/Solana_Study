
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BatchExecuteCtxekcr<'info> {
    #[account(mut)] pub caller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_099 {
    use super::*;

    pub fn batch_execute(ctx: Context<BatchExecuteCtxekcr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.caller;
        // custom logic for batch_execute
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed batch_execute logic");
        Ok(())
    }
}
