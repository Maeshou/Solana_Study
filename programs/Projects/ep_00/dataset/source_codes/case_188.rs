use anchor_lang::prelude::*;

declare_id!("d0Q1UAGrhqBGTWDyupKFnMZrdbH2Hfa4iJZWzOH3FhAL");

#[derive(Accounts)]
pub struct Case188<'info> {
    #[account(mut, has_one = owner47)] pub acct92: Account<'info, DataAccount>,
    #[account(mut)] pub acct70: Account<'info, DataAccount>,
    pub owner47: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_188_program {
    use super::*;

    pub fn case_188(ctx: Context<Case188>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner47.data;
        ctx.accounts.acct92.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
