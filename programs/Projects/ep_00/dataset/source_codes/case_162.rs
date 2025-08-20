use anchor_lang::prelude::*;

declare_id!("qfhkLoENljnwmVMTXRa3ei6rm7g2qfDbtGwi7vrnkxgK");

#[derive(Accounts)]
pub struct Case162<'info> {
    #[account(mut, has_one = owner41)] pub acct91: Account<'info, DataAccount>,
    #[account(mut)] pub acct3: Account<'info, DataAccount>,
    #[account(mut)] pub acct36: Account<'info, DataAccount>,
    pub owner41: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_162_program {
    use super::*;

    pub fn case_162(ctx: Context<Case162>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct91.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct91.data = new_balance;
        Ok(())
    }
}
