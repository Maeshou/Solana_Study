use anchor_lang::prelude::*;

declare_id!("zkSCAv0vgGQlbsL3hHzyV3opWKOv7mC72cHyVKEr9rdt");

#[derive(Accounts)]
pub struct Case149<'info> {
    #[account(mut, has_one = owner6)] pub acct96: Account<'info, DataAccount>,
    #[account(mut)] pub acct55: Account<'info, DataAccount>,
    #[account(mut)] pub acct86: Account<'info, DataAccount>,
    pub owner6: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_149_program {
    use super::*;

    pub fn case_149(ctx: Context<Case149>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct96.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct96.data = new_balance;
        Ok(())
    }
}
