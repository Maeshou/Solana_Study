use anchor_lang::prelude::*;

declare_id!("sri0D8aBkE5pcSYpjtOuN5iXWuJNlIAaMg94tWrnmHdi");

#[derive(Accounts)]
pub struct Case189<'info> {
    #[account(mut, has_one = owner1)] pub acct19: Account<'info, DataAccount>,
    pub owner1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_189_program {
    use super::*;

    pub fn case_189(ctx: Context<Case189>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner1.data;
        ctx.accounts.acct19.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
