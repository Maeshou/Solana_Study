use anchor_lang::prelude::*;

declare_id!("nQn6FyCM79z6de1PB72BgxY2oXVOBStSPtNmSO8Liakg");

#[derive(Accounts)]
pub struct Case182<'info> {
    #[account(mut, has_one = owner15)] pub acct4: Account<'info, DataAccount>,
    pub owner15: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_182_program {
    use super::*;

    pub fn case_182(ctx: Context<Case182>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct4.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct4.data = new_balance;
        Ok(())
    }
}
