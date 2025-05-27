
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetOracleCtxvrmr<'info> {
    #[account(mut)] pub oracle: Account<'info, DataAccount>,
    #[account(mut)] pub updater: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_083 {
    use super::*;

    pub fn set_oracle(ctx: Context<SetOracleCtxvrmr>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.oracle;
        // custom logic for set_oracle
        **ctx.accounts.oracle.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed set_oracle logic");
        Ok(())
    }
}
