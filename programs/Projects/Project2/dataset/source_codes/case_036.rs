
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateGovernanceCtxmglj<'info> {
    #[account(mut)] pub governance: Account<'info, DataAccount>,
    #[account(mut)] pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_036 {
    use super::*;

    pub fn update_governance(ctx: Context<UpdateGovernanceCtxmglj>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.governance;
        // custom logic for update_governance
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed update_governance logic");
        Ok(())
    }
}
