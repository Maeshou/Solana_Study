use anchor_lang::prelude::*;

declare_id!("YN3IgOCu50JfH4E44QuRpgaJsYnzwzQJ4FIuXdQ7iTn7");

#[derive(Accounts)]
pub struct Case136<'info> {
    #[account(mut, has_one = owner1)] pub acct84: Account<'info, DataAccount>,
    #[account(mut)] pub acct5: Account<'info, DataAccount>,
    #[account(mut)] pub acct23: Account<'info, DataAccount>,
    pub owner1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_136_program {
    use super::*;

    pub fn case_136(ctx: Context<Case136>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct84.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct84.data = tripled;
        Ok(())
    }
}
