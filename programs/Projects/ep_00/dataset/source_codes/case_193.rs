use anchor_lang::prelude::*;

declare_id!("T65iKUAoqE5ddCQ4BVugbqJPjnqR4p9wkxjb1j2YlTgU");

#[derive(Accounts)]
pub struct Case193<'info> {
    #[account(mut, has_one = owner27)] pub acct41: Account<'info, DataAccount>,
    #[account(mut)] pub acct92: Account<'info, DataAccount>,
    #[account(mut)] pub acct49: Account<'info, DataAccount>,
    pub owner27: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_193_program {
    use super::*;

    pub fn case_193(ctx: Context<Case193>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct41.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct41.data = tripled;
        Ok(())
    }
}
