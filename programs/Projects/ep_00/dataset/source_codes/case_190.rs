use anchor_lang::prelude::*;

declare_id!("rN7d1RJRFtZMG2WGs7E54ycqR4qbwIISap5CSx4kMOKa");

#[derive(Accounts)]
pub struct Case190<'info> {
    #[account(mut, has_one = owner37)] pub acct44: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct63: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_190_program {
    use super::*;

    pub fn case_190(ctx: Context<Case190>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner37.data;
        ctx.accounts.acct44.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
