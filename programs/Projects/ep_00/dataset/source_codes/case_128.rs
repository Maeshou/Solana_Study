use anchor_lang::prelude::*;

declare_id!("r9SS2pFeNvd23AkIuGK9WWqfaav9Mw1W5zlEDSCjcW6t");

#[derive(Accounts)]
pub struct Case128<'info> {
    #[account(mut, has_one = owner22)] pub acct24: Account<'info, DataAccount>,
    pub owner22: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_128_program {
    use super::*;

    pub fn case_128(ctx: Context<Case128>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct24.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct24.data = tripled;
        Ok(())
    }
}
