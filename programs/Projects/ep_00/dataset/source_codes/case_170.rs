use anchor_lang::prelude::*;

declare_id!("jJXELTVWx0kKQz6fjaGeBuuERwc887PWjsjNPpXBVcBD");

#[derive(Accounts)]
pub struct Case170<'info> {
    #[account(mut, has_one = owner13)] pub acct77: Account<'info, DataAccount>,
    #[account(mut)] pub acct28: Account<'info, DataAccount>,
    #[account(mut)] pub acct84: Account<'info, DataAccount>,
    pub owner13: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_170_program {
    use super::*;

    pub fn case_170(ctx: Context<Case170>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner13.data;
        ctx.accounts.acct77.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
