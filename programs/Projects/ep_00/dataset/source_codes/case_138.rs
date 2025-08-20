use anchor_lang::prelude::*;

declare_id!("ZOMKDJcyLsaUZ0Pr4RGB9NpGStX6zMMvdq43ka2TRKnN");

#[derive(Accounts)]
pub struct Case138<'info> {
    #[account(mut, has_one = owner37)] pub acct59: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_138_program {
    use super::*;

    pub fn case_138(ctx: Context<Case138>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct59.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct59.data = tripled;
        Ok(())
    }
}
