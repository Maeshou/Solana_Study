use anchor_lang::prelude::*;

declare_id!("ciCQqSC5GGvLwSRdIqDkRYhP8fUtZZyXXRTHNxIHe4PE");

#[derive(Accounts)]
pub struct Case192<'info> {
    #[account(mut, has_one = owner9)] pub acct7: Account<'info, DataAccount>,
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    pub owner9: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_192_program {
    use super::*;

    pub fn case_192(ctx: Context<Case192>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner9.data;
        ctx.accounts.acct7.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
