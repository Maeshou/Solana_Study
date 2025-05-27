
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UndelegateCtxtaek<'info> {
    #[account(mut)] pub delegation: Account<'info, DataAccount>,
    #[account(mut)] pub delegator: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_038 {
    use super::*;

    pub fn undelegate(ctx: Context<UndelegateCtxtaek>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.delegation;
        // custom logic for undelegate
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed undelegate logic");
        Ok(())
    }
}
