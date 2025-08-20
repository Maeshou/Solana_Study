use anchor_lang::prelude::*;

declare_id!("et1trFshJWljfCOCg2KB38UGpmWYOXLwC5k9Bb1HmHkr");

#[derive(Accounts)]
pub struct Case127<'info> {
    #[account(mut, has_one = owner35)] pub acct8: Account<'info, DataAccount>,
    #[account(mut)] pub acct85: Account<'info, DataAccount>,
    pub owner35: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_127_program {
    use super::*;

    pub fn case_127(ctx: Context<Case127>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct8.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct8.data = tripled;
        Ok(())
    }
}
