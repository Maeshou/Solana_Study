use anchor_lang::prelude::*;

declare_id!("4ceKmLbz6eO32TF54DEZy5mGl6uVDZKlryLxQgxE45g6");

#[derive(Accounts)]
pub struct Case103<'info> {
    #[account(mut, has_one = owner15)] pub acct59: Account<'info, DataAccount>,
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    #[account(mut)] pub acct45: Account<'info, DataAccount>,
    pub owner15: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_103_program {
    use super::*;

    pub fn case_103(ctx: Context<Case103>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct59.data = set_val;
        Ok(())
    }
}
