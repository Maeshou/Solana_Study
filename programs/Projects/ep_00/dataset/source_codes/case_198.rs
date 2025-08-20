use anchor_lang::prelude::*;

declare_id!("8dxpGiSwrFJLCrIla5MTmLAfaGOzb27bC9VryHsfWcZK");

#[derive(Accounts)]
pub struct Case198<'info> {
    #[account(mut, has_one = owner37)] pub acct18: Account<'info, DataAccount>,
    #[account(mut)] pub acct95: Account<'info, DataAccount>,
    pub owner37: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_198_program {
    use super::*;

    pub fn case_198(ctx: Context<Case198>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner37.data;
        ctx.accounts.acct18.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
