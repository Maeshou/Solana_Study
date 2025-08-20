use anchor_lang::prelude::*;

declare_id!("ZKXGwvyHH0L1UiWhx6z7wBNaPEztO9ahkTxqcNJnrr8d");

#[derive(Accounts)]
pub struct Case135<'info> {
    #[account(mut, has_one = owner37)] pub acct35: Account<'info, DataAccount>,
    #[account(mut)] pub acct50: Account<'info, DataAccount>,
    #[account(mut)] pub acct46: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_135_program {
    use super::*;

    pub fn case_135(ctx: Context<Case135>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct35.data = set_val;
        Ok(())
    }
}
