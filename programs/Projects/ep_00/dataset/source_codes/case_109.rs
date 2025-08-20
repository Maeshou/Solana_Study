use anchor_lang::prelude::*;

declare_id!("B8ZMiAIj1fgiLZbXvh6GlnH1NLYTOENe9ffESm6XdMvr");

#[derive(Accounts)]
pub struct Case109<'info> {
    #[account(mut, has_one = owner49)] pub acct73: Account<'info, DataAccount>,
    pub owner49: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_109_program {
    use super::*;

    pub fn case_109(ctx: Context<Case109>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct73.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct73.data = tripled;
        Ok(())
    }
}
