use anchor_lang::prelude::*;

declare_id!("gdd4vGUmOD35sRNCgI7tP7QF7au0G9uny2ABfyBMsyYc");

#[derive(Accounts)]
pub struct Case184<'info> {
    #[account(mut, has_one = owner7)] pub acct22: Account<'info, DataAccount>,
    pub owner7: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_184_program {
    use super::*;

    pub fn case_184(ctx: Context<Case184>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct22.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct22.data = tripled;
        Ok(())
    }
}
