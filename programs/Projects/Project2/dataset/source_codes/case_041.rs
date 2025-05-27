
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CancelEscrowCtxelwl<'info> {
    #[account(mut)] pub escrow: Account<'info, DataAccount>,
    #[account(mut)] pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_041 {
    use super::*;

    pub fn cancel_escrow(ctx: Context<CancelEscrowCtxelwl>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.escrow;
        // custom logic for cancel_escrow
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed cancel_escrow logic");
        Ok(())
    }
}
