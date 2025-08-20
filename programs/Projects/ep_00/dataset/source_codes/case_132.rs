use anchor_lang::prelude::*;

declare_id!("OZJHQQ9fNqMjV0ghEeca0tX5Uu0sW8VtHqx8qhEvZneq");

#[derive(Accounts)]
pub struct Case132<'info> {
    #[account(mut, has_one = owner39)] pub acct26: Account<'info, DataAccount>,
    #[account(mut)] pub acct39: Account<'info, DataAccount>,
    #[account(mut)] pub acct6: Account<'info, DataAccount>,
    pub owner39: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_132_program {
    use super::*;

    pub fn case_132(ctx: Context<Case132>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner39.data;
        ctx.accounts.acct26.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
