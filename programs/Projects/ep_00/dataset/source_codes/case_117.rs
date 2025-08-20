use anchor_lang::prelude::*;

declare_id!("H7P9S4dcsbdVhAqEF4ig8pxKZggs5E3BsE0UWxSLE3jW");

#[derive(Accounts)]
pub struct Case117<'info> {
    #[account(mut, has_one = owner19)] pub acct24: Account<'info, DataAccount>,
    #[account(mut)] pub acct21: Account<'info, DataAccount>,
    #[account(mut)] pub acct99: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_117_program {
    use super::*;

    pub fn case_117(ctx: Context<Case117>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct24.data = set_val;
        Ok(())
    }
}
