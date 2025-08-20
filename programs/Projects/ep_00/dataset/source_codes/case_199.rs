use anchor_lang::prelude::*;

declare_id!("hJhGQCAFRwG1dGLZbdGEDYoA3Dfe4ALQKbAW7XmJ3xu0");

#[derive(Accounts)]
pub struct Case199<'info> {
    #[account(mut, has_one = owner15)] pub acct7: Account<'info, DataAccount>,
    #[account(mut)] pub acct8: Account<'info, DataAccount>,
    #[account(mut)] pub acct77: Account<'info, DataAccount>,
    pub owner15: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_199_program {
    use super::*;

    pub fn case_199(ctx: Context<Case199>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct7.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct7.data = tripled;
        Ok(())
    }
}
