use anchor_lang::prelude::*;

declare_id!("5cgCVBgtzf4WT6NVqRENvIHuqi5dCdrkHui9RQjAVh1h");

#[derive(Accounts)]
pub struct Case130<'info> {
    #[account(mut, has_one = owner44)] pub acct17: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    pub owner44: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_130_program {
    use super::*;

    pub fn case_130(ctx: Context<Case130>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner44.data;
        ctx.accounts.acct17.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
