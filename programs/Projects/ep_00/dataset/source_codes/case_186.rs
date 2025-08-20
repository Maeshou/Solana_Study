use anchor_lang::prelude::*;

declare_id!("wKpZyLPNziTIOluDx7Z6Hy5ZFGyXLKZz71T8LbatfQZF");

#[derive(Accounts)]
pub struct Case186<'info> {
    #[account(mut, has_one = owner19)] pub acct67: Account<'info, DataAccount>,
    #[account(mut)] pub acct78: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_186_program {
    use super::*;

    pub fn case_186(ctx: Context<Case186>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct67.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct67.data = new_balance;
        Ok(())
    }
}
