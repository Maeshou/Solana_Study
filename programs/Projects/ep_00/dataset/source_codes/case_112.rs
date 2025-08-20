use anchor_lang::prelude::*;

declare_id!("K6rMGYOMnIk9qYKtaxzoT1mBBlKcFVCY2nyd7EB3smyT");

#[derive(Accounts)]
pub struct Case112<'info> {
    #[account(mut, has_one = owner33)] pub acct81: Account<'info, DataAccount>,
    #[account(mut)] pub acct37: Account<'info, DataAccount>,
    #[account(mut)] pub acct72: Account<'info, DataAccount>,
    pub owner33: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_112_program {
    use super::*;

    pub fn case_112(ctx: Context<Case112>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner33.data;
        ctx.accounts.acct81.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
