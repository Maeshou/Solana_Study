use anchor_lang::prelude::*;

declare_id!("y9kaDTnOpdzEceqXCQUlQpMc2GcnspL3NZokRLFBvB21");

#[derive(Accounts)]
pub struct Case143<'info> {
    #[account(mut, has_one = owner26)] pub acct28: Account<'info, DataAccount>,
    #[account(mut)] pub acct71: Account<'info, DataAccount>,
    #[account(mut)] pub acct51: Account<'info, DataAccount>,
    pub owner26: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_143_program {
    use super::*;

    pub fn case_143(ctx: Context<Case143>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct28.data = set_val;
        Ok(())
    }
}
