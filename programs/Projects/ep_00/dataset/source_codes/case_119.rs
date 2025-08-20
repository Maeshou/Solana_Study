use anchor_lang::prelude::*;

declare_id!("t6uOf2XukETDv6CbpiWUfP6drHPcP0S7DNH9I3v3YQVf");

#[derive(Accounts)]
pub struct Case119<'info> {
    #[account(mut, has_one = owner23)] pub acct91: Account<'info, DataAccount>,
    pub owner23: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_119_program {
    use super::*;

    pub fn case_119(ctx: Context<Case119>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner23.data;
        ctx.accounts.acct91.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
