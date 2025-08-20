use anchor_lang::prelude::*;

declare_id!("prmPFltjdIpDbyYxNk1njuP7XKzYMHEWEAaY21F0YGTb");

#[derive(Accounts)]
pub struct Case181<'info> {
    #[account(mut, has_one = owner39)] pub acct93: Account<'info, DataAccount>,
    #[account(mut)] pub acct1: Account<'info, DataAccount>,
    #[account(mut)] pub acct43: Account<'info, DataAccount>,
    pub owner39: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_181_program {
    use super::*;

    pub fn case_181(ctx: Context<Case181>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct93.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct93.data = new_balance;
        Ok(())
    }
}
