use anchor_lang::prelude::*;

declare_id!("7adpcPO59zabFN2IqMomIrLEZlwA21vLwS0lVuHeMJ33");

#[derive(Accounts)]
pub struct Case166<'info> {
    #[account(mut, has_one = owner26)] pub acct33: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    pub owner26: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_166_program {
    use super::*;

    pub fn case_166(ctx: Context<Case166>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct33.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct33.data = tripled;
        Ok(())
    }
}
