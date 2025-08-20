use anchor_lang::prelude::*;

declare_id!("yyIbl40MPOwZCv1iBg90RULQJGVQa9Au8IxlCf4TLx8I");

#[derive(Accounts)]
pub struct Case183<'info> {
    #[account(mut, has_one = owner36)] pub acct22: Account<'info, DataAccount>,
    #[account(mut)] pub acct33: Account<'info, DataAccount>,
    pub owner36: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_183_program {
    use super::*;

    pub fn case_183(ctx: Context<Case183>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct22.data = set_val;
        Ok(())
    }
}
