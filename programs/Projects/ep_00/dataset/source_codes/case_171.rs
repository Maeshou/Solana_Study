use anchor_lang::prelude::*;

declare_id!("1foTPHkstcnY0tX9z315UpqPcJ7ctARB2XbHabM8tKKI");

#[derive(Accounts)]
pub struct Case171<'info> {
    #[account(mut, has_one = owner19)] pub acct22: Account<'info, DataAccount>,
    #[account(mut)] pub acct28: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_171_program {
    use super::*;

    pub fn case_171(ctx: Context<Case171>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner19.data;
        ctx.accounts.acct22.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
