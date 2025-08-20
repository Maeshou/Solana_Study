use anchor_lang::prelude::*;

declare_id!("GqYVEkS9TKppsAJ5IjPNOE8H7jVNMVUeSfK0BmXs9EQw");

#[derive(Accounts)]
pub struct Case185<'info> {
    #[account(mut, has_one = owner7)] pub acct57: Account<'info, DataAccount>,
    #[account(mut)] pub acct91: Account<'info, DataAccount>,
    pub owner7: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_185_program {
    use super::*;

    pub fn case_185(ctx: Context<Case185>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct57.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct57.data = tripled;
        Ok(())
    }
}
