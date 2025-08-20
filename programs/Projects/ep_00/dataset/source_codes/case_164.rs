use anchor_lang::prelude::*;

declare_id!("G3eagsXdNQiWnN7KKSJbxWTQHPT0zLvewBCPJVBQjEIK");

#[derive(Accounts)]
pub struct Case164<'info> {
    #[account(mut, has_one = owner8)] pub acct23: Account<'info, DataAccount>,
    #[account(mut)] pub acct13: Account<'info, DataAccount>,
    #[account(mut)] pub acct43: Account<'info, DataAccount>,
    pub owner8: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_164_program {
    use super::*;

    pub fn case_164(ctx: Context<Case164>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner8.data;
        ctx.accounts.acct23.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
