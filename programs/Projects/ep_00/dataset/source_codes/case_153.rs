use anchor_lang::prelude::*;

declare_id!("989IPv0D4eoAqgsAAIxctuI20G8FOByFCVQQU9KZ0Se0");

#[derive(Accounts)]
pub struct Case153<'info> {
    #[account(mut, has_one = owner22)] pub acct20: Account<'info, DataAccount>,
    #[account(mut)] pub acct89: Account<'info, DataAccount>,
    #[account(mut)] pub acct75: Account<'info, DataAccount>,
    pub owner22: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_153_program {
    use super::*;

    pub fn case_153(ctx: Context<Case153>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner22.data;
        ctx.accounts.acct20.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
