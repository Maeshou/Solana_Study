use anchor_lang::prelude::*;

declare_id!("XSl5D68kO5d1WPm16mCn1Zpc9ZWufHwldhZwPNBZc4pv");

#[derive(Accounts)]
pub struct Case169<'info> {
    #[account(mut, has_one = owner36)] pub acct35: Account<'info, DataAccount>,
    #[account(mut)] pub acct92: Account<'info, DataAccount>,
    #[account(mut)] pub acct84: Account<'info, DataAccount>,
    pub owner36: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_169_program {
    use super::*;

    pub fn case_169(ctx: Context<Case169>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct35.data = set_val;
        Ok(())
    }
}
