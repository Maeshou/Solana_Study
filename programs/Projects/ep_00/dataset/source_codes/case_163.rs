use anchor_lang::prelude::*;

declare_id!("5DLZzWwx3n2rqTXjtzQDMubZWPPkEgoM3t2DFzTeUlju");

#[derive(Accounts)]
pub struct Case163<'info> {
    #[account(mut, has_one = owner30)] pub acct12: Account<'info, DataAccount>,
    pub owner30: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_163_program {
    use super::*;

    pub fn case_163(ctx: Context<Case163>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct12.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct12.data = tripled;
        Ok(())
    }
}
