
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RebalanceCtxnuvn<'info> {
    #[account(mut)] pub portfolio: Account<'info, DataAccount>,
    #[account(mut)] pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_094 {
    use super::*;

    pub fn rebalance(ctx: Context<RebalanceCtxnuvn>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.portfolio;
        // custom logic for rebalance
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed rebalance logic");
        Ok(())
    }
}
