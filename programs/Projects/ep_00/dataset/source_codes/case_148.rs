use anchor_lang::prelude::*;

declare_id!("PQ8O5PjZ38jT5jeNbou26h44YjSwBfFSwG1mCCEj4x5J");

#[derive(Accounts)]
pub struct Case148<'info> {
    #[account(mut, has_one = owner15)] pub acct66: Account<'info, DataAccount>,
    #[account(mut)] pub acct25: Account<'info, DataAccount>,
    #[account(mut)] pub acct88: Account<'info, DataAccount>,
    pub owner15: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_148_program {
    use super::*;

    pub fn case_148(ctx: Context<Case148>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct66.data = set_val;
        Ok(())
    }
}
