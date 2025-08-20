use anchor_lang::prelude::*;

declare_id!("2nTEue3vaBKnAuxz27uI5jJW8D4fwYmgYS3OEYO2lWwf");

#[derive(Accounts)]
pub struct Case177<'info> {
    #[account(mut, has_one = owner17)] pub acct43: Account<'info, DataAccount>,
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    #[account(mut)] pub acct16: Account<'info, DataAccount>,
    pub owner17: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_177_program {
    use super::*;

    pub fn case_177(ctx: Context<Case177>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct43.data = set_val;
        Ok(())
    }
}
