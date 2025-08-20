use anchor_lang::prelude::*;

declare_id!("ZGP3eSFM3Tzj2RFy0zy8ljS7kkdJ61Ix3lhlkO2oJHGJ");

#[derive(Accounts)]
pub struct Case123<'info> {
    #[account(mut, has_one = owner40)] pub acct93: Account<'info, DataAccount>,
    pub owner40: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_123_program {
    use super::*;

    pub fn case_123(ctx: Context<Case123>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct93.data = set_val;
        Ok(())
    }
}
