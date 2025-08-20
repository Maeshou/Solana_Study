use anchor_lang::prelude::*;

declare_id!("PyILSWDnTFsUo4nUQiliuJq1qY4mUG7CMPxNrxgd7Jqe");

#[derive(Accounts)]
pub struct Case159<'info> {
    #[account(mut, has_one = owner41)] pub acct96: Account<'info, DataAccount>,
    pub owner41: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_159_program {
    use super::*;

    pub fn case_159(ctx: Context<Case159>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct96.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct96.data = tripled;
        Ok(())
    }
}
