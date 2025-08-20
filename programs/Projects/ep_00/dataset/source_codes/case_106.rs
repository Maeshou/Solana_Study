use anchor_lang::prelude::*;

declare_id!("Pn04pqmvx7rXte1L0BtkoWpNsRuUlDdfuNrH6ikN7Se7");

#[derive(Accounts)]
pub struct Case106<'info> {
    #[account(mut, has_one = owner7)] pub acct2: Account<'info, DataAccount>,
    pub owner7: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_106_program {
    use super::*;

    pub fn case_106(ctx: Context<Case106>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct2.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct2.data = tripled;
        Ok(())
    }
}
