use anchor_lang::prelude::*;

declare_id!("aF49TspRkqhjwmmC3lXn5Tfd1SF9DSYGh4Y9DhJ9lS85");

#[derive(Accounts)]
pub struct Case158<'info> {
    #[account(mut, has_one = owner37)] pub acct26: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_158_program {
    use super::*;

    pub fn case_158(ctx: Context<Case158>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct26.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct26.data = result;
        Ok(())
    }
}
