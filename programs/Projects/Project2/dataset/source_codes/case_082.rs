
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CrossTransferCtxnunz<'info> {
    #[account(mut)] pub from: Account<'info, DataAccount>,
    #[account(mut)] pub to: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_082 {
    use super::*;

    pub fn cross_transfer(ctx: Context<CrossTransferCtxnunz>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.from;
        // custom logic for cross_transfer
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed cross_transfer logic");
        Ok(())
    }
}
