use anchor_lang::prelude::*;

declare_id!("IXPG8oz9XJdIyspkP1MwkCg18ZffxaFVbvX8xYSY3eP5");

#[derive(Accounts)]
pub struct Case195<'info> {
    #[account(mut, has_one = owner20)] pub acct42: Account<'info, DataAccount>,
    #[account(mut)] pub acct36: Account<'info, DataAccount>,
    #[account(mut)] pub acct46: Account<'info, DataAccount>,
    pub owner20: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_195_program {
    use super::*;

    pub fn case_195(ctx: Context<Case195>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct42.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct42.data = new_balance;
        Ok(())
    }
}
