use anchor_lang::prelude::*;

declare_id!("BWZ1O0NrUq8zhIw9ZC36KYfeS16BEGUX4evV2sTbLaOZ");

#[derive(Accounts)]
pub struct Case101<'info> {
    #[account(mut, has_one = owner7)] pub acct20: Account<'info, DataAccount>,
    #[account(mut)] pub acct31: Account<'info, DataAccount>,
    #[account(mut)] pub acct60: Account<'info, DataAccount>,
    pub owner7: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_101_program {
    use super::*;

    pub fn case_101(ctx: Context<Case101>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner7.data;
        ctx.accounts.acct20.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
