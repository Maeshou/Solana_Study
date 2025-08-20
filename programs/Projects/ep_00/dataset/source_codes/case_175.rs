use anchor_lang::prelude::*;

declare_id!("U5peOp69Lwpqh9c4xGOYDP5BoWkjdvV4EP7Xvio2VdN3");

#[derive(Accounts)]
pub struct Case175<'info> {
    #[account(mut, has_one = owner46)] pub acct34: Account<'info, DataAccount>,
    #[account(mut)] pub acct12: Account<'info, DataAccount>,
    pub owner46: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_175_program {
    use super::*;

    pub fn case_175(ctx: Context<Case175>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct34.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct34.data = tripled;
        Ok(())
    }
}
