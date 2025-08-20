use anchor_lang::prelude::*;

declare_id!("SNHPto97ACbswGRXoQU9oI1wR1u2bFIhxWKVbyKomzFQ");

#[derive(Accounts)]
pub struct Case145<'info> {
    #[account(mut, has_one = owner11)] pub acct38: Account<'info, DataAccount>,
    #[account(mut)] pub acct45: Account<'info, DataAccount>,
    #[account(mut)] pub acct94: Account<'info, DataAccount>,
    pub owner11: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_145_program {
    use super::*;

    pub fn case_145(ctx: Context<Case145>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct38.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct38.data = result;
        Ok(())
    }
}
