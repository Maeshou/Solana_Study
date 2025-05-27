
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferCtxlcas<'info> {
    #[account(mut)] pub source_account: Account<'info, DataAccount>,
    #[account(mut)] pub dest_account: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_004 {
    use super::*;

    pub fn transfer(ctx: Context<TransferCtxlcas>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.source_account;
        // custom logic for transfer
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed transfer logic");
        Ok(())
    }
}
