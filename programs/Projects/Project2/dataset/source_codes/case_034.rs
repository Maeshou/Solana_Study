
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExecuteProposalCtxcfpo<'info> {
    #[account(mut)] pub proposal: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_034 {
    use super::*;

    pub fn execute_proposal(ctx: Context<ExecuteProposalCtxcfpo>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.proposal;
        // custom logic for execute_proposal
        acct.data = acct.data.checked_add(amount).unwrap();
        msg!("Executed execute_proposal logic");
        Ok(())
    }
}
