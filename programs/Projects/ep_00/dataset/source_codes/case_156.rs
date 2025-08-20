use anchor_lang::prelude::*;

declare_id!("YOlkesUyg8BcBnGgrHbujP8jOP3H3FLrCZpqz74LemWx");

#[derive(Accounts)]
pub struct Case156<'info> {
    #[account(mut, has_one = owner1)] pub acct52: Account<'info, DataAccount>,
    #[account(mut)] pub acct88: Account<'info, DataAccount>,
    pub owner1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_156_program {
    use super::*;

    pub fn case_156(ctx: Context<Case156>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct52.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct52.data = new_balance;
        Ok(())
    }
}
