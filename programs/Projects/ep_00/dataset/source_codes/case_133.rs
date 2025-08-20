use anchor_lang::prelude::*;

declare_id!("x0oxviwzkhjHDMRe0TpgHWomkt4m143v5aU7kZMU2jta");

#[derive(Accounts)]
pub struct Case133<'info> {
    #[account(mut, has_one = owner35)] pub acct90: Account<'info, DataAccount>,
    pub owner35: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_133_program {
    use super::*;

    pub fn case_133(ctx: Context<Case133>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct90.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct90.data = new_balance;
        Ok(())
    }
}
