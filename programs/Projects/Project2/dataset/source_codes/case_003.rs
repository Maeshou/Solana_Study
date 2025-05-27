
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferCtxwgjm<'info> {
    #[account(mut)] pub from: Account<'info, DataAccount>,
    #[account(mut)] pub to: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_003 {
    use super::*;

    pub fn transfer(ctx: Context<TransferCtxwgjm>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.from;
        // custom logic for transfer
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed transfer logic");
        Ok(())
    }
}
