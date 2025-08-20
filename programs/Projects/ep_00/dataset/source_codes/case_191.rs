use anchor_lang::prelude::*;

declare_id!("QE67Nba8IUC9STpwoAJ7lovvY5Ny9OWAF6e8AU2UcV9B");

#[derive(Accounts)]
pub struct Case191<'info> {
    #[account(mut, has_one = owner27)] pub acct1: Account<'info, DataAccount>,
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    pub owner27: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_191_program {
    use super::*;

    pub fn case_191(ctx: Context<Case191>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct1.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct1.data = tripled;
        Ok(())
    }
}
