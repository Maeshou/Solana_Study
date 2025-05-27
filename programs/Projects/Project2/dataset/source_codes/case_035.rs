
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CancelProposalCtxyncs<'info> {
    #[account(mut)] pub proposal: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_035 {
    use super::*;

    pub fn cancel_proposal(ctx: Context<CancelProposalCtxyncs>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.proposal;
        // custom logic for cancel_proposal
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed cancel_proposal logic");
        Ok(())
    }
}
