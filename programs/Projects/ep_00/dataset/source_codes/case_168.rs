use anchor_lang::prelude::*;

declare_id!("pwsYVujEjEbyOyZG5oPs55HBAeFuBj7v35EiAFP76EvH");

#[derive(Accounts)]
pub struct Case168<'info> {
    #[account(mut, has_one = owner41)] pub acct13: Account<'info, DataAccount>,
    #[account(mut)] pub acct87: Account<'info, DataAccount>,
    pub owner41: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_168_program {
    use super::*;

    pub fn case_168(ctx: Context<Case168>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner41.data;
        ctx.accounts.acct13.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
