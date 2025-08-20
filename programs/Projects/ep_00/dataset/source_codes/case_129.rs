use anchor_lang::prelude::*;

declare_id!("o0twhxhQivY4x6TGHwUnUnqggsdHutlVpg2QkUDsA4tP");

#[derive(Accounts)]
pub struct Case129<'info> {
    #[account(mut, has_one = owner3)] pub acct54: Account<'info, DataAccount>,
    #[account(mut)] pub acct74: Account<'info, DataAccount>,
    pub owner3: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_129_program {
    use super::*;

    pub fn case_129(ctx: Context<Case129>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner3.data;
        ctx.accounts.acct54.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
