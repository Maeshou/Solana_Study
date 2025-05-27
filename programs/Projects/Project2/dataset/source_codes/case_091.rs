
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct BuyOptionCtxwpel<'info> {
    #[account(mut)] pub option: Account<'info, DataAccount>,
    #[account(mut)] pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_091 {
    use super::*;

    pub fn buy_option(ctx: Context<BuyOptionCtxwpel>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.option;
        // custom logic for buy_option
        assert!(ctx.accounts.option.data > 0); acct.data -= amount;
        msg!("Executed buy_option logic");
        Ok(())
    }
}
