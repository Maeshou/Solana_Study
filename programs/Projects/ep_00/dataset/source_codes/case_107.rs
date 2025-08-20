use anchor_lang::prelude::*;

declare_id!("h7bdpYWT2NcNPozkwJ2ULjtmPOGUVqEBSZ9E9teK5hye");

#[derive(Accounts)]
pub struct Case107<'info> {
    #[account(mut, has_one = owner44)] pub acct36: Account<'info, DataAccount>,
    #[account(mut)] pub acct31: Account<'info, DataAccount>,
    #[account(mut)] pub acct70: Account<'info, DataAccount>,
    pub owner44: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_107_program {
    use super::*;

    pub fn case_107(ctx: Context<Case107>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct36.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct36.data = tripled;
        Ok(())
    }
}
