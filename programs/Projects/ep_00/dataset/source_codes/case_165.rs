use anchor_lang::prelude::*;

declare_id!("uAfMSutAlb9egpF9yIu69m8mJ7VO6JfWzqrd1VI4ZgKf");

#[derive(Accounts)]
pub struct Case165<'info> {
    #[account(mut, has_one = owner44)] pub acct6: Account<'info, DataAccount>,
    pub owner44: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_165_program {
    use super::*;

    pub fn case_165(ctx: Context<Case165>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct6.data = set_val;
        Ok(())
    }
}
