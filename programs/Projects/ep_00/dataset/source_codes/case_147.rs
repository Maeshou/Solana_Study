use anchor_lang::prelude::*;

declare_id!("VHSkDizTvRziguGAbHeUG0lWQ4cFNZ9ODxVZtedLHND9");

#[derive(Accounts)]
pub struct Case147<'info> {
    #[account(mut, has_one = owner31)] pub acct100: Account<'info, DataAccount>,
    #[account(mut)] pub acct25: Account<'info, DataAccount>,
    pub owner31: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_147_program {
    use super::*;

    pub fn case_147(ctx: Context<Case147>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct100.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct100.data = tripled;
        Ok(())
    }
}
