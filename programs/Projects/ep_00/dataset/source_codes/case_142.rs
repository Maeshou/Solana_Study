use anchor_lang::prelude::*;

declare_id!("tPFITdNuta2cmVbqyvbu6GGCol8weP8sestdVlrO6q9e");

#[derive(Accounts)]
pub struct Case142<'info> {
    #[account(mut, has_one = owner32)] pub acct14: Account<'info, DataAccount>,
    pub owner32: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_142_program {
    use super::*;

    pub fn case_142(ctx: Context<Case142>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct14.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct14.data = tripled;
        Ok(())
    }
}
