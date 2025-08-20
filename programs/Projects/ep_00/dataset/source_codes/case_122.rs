use anchor_lang::prelude::*;

declare_id!("Zk31CfGzhzrvUMO4v0TMjlqSVcEwapSU67frHYLw6Fnv");

#[derive(Accounts)]
pub struct Case122<'info> {
    #[account(mut, has_one = owner27)] pub acct66: Account<'info, DataAccount>,
    pub owner27: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_122_program {
    use super::*;

    pub fn case_122(ctx: Context<Case122>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner27.data;
        ctx.accounts.acct66.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
