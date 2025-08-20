use anchor_lang::prelude::*;

declare_id!("w48Ho0acdQAMLqspqfzOBSdQ7d0rgDckTpG9RJKBQ9Cn");

#[derive(Accounts)]
pub struct Case155<'info> {
    #[account(mut, has_one = owner22)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct35: Account<'info, DataAccount>,
    pub owner22: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_155_program {
    use super::*;

    pub fn case_155(ctx: Context<Case155>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct55.data = set_val;
        Ok(())
    }
}
