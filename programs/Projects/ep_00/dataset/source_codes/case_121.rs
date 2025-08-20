use anchor_lang::prelude::*;

declare_id!("Ys3OREoOui8hV9XDRkaCSjWbOTZN0PgW0SQVgxeHgtDG");

#[derive(Accounts)]
pub struct Case121<'info> {
    #[account(mut, has_one = owner9)] pub acct63: Account<'info, DataAccount>,
    #[account(mut)] pub acct53: Account<'info, DataAccount>,
    #[account(mut)] pub acct42: Account<'info, DataAccount>,
    pub owner9: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_121_program {
    use super::*;

    pub fn case_121(ctx: Context<Case121>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct63.data = set_val;
        Ok(())
    }
}
