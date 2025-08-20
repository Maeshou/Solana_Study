use anchor_lang::prelude::*;

declare_id!("r3MoOjlLKDlHJYWqqh5DNcYJ1tfXN18PaJ92KzVgrzy8");

#[derive(Accounts)]
pub struct Case105<'info> {
    #[account(mut, has_one = owner46)] pub acct1: Account<'info, DataAccount>,
    #[account(mut)] pub acct61: Account<'info, DataAccount>,
    #[account(mut)] pub acct64: Account<'info, DataAccount>,
    pub owner46: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_105_program {
    use super::*;

    pub fn case_105(ctx: Context<Case105>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let owner_val = ctx.accounts.owner46.data;
        ctx.accounts.acct1.data = owner_val.checked_sub(1).unwrap();
        Ok(())
    }
}
