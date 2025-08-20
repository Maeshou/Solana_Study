use anchor_lang::prelude::*;

declare_id!("BI4hL2yO0CxhcfitUIPBnnflVfvTCjFalIY1U8uTpDVL");

#[derive(Accounts)]
pub struct Case134<'info> {
    #[account(mut, has_one = owner12)] pub acct4: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct50: Account<'info, DataAccount>,
    pub owner12: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_134_program {
    use super::*;

    pub fn case_134(ctx: Context<Case134>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner12.data;
        ctx.accounts.acct4.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
