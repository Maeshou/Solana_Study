use anchor_lang::prelude::*;

declare_id!("ICJw8T83ILXDj1leOwPih6Ky5Vozf2WHiaTUlqGi1KVB");

#[derive(Accounts)]
pub struct Case174<'info> {
    #[account(mut, has_one = owner47)] pub acct24: Account<'info, DataAccount>,
    #[account(mut)] pub acct67: Account<'info, DataAccount>,
    pub owner47: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_174_program {
    use super::*;

    pub fn case_174(ctx: Context<Case174>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct24.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct24.data = tripled;
        Ok(())
    }
}
