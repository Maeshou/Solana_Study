
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RepayCtxpyxm<'info> {
    #[account(mut)] pub reserve: Account<'info, DataAccount>,
    #[account(mut)] pub payer: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_013 {
    use super::*;

    pub fn repay(ctx: Context<RepayCtxpyxm>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.reserve;
        // custom logic for repay
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed repay logic");
        Ok(())
    }
}
