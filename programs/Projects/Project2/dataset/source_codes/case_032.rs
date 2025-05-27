
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeGovernanceCtxqtzn<'info> {
    #[account(mut)] pub governance: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_032 {
    use super::*;

    pub fn initialize_governance(ctx: Context<InitializeGovernanceCtxqtzn>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.governance;
        // custom logic for initialize_governance
        **ctx.accounts.governance.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Executed initialize_governance logic");
        Ok(())
    }
}
