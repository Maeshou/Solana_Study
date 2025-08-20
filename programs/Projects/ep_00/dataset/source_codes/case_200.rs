use anchor_lang::prelude::*;

declare_id!("wMoxAe0v04ibw8uYaWXwixApbbqySUdBJwxvcNAK3gAQ");

#[derive(Accounts)]
pub struct Case200<'info> {
    #[account(mut, has_one = owner16)] pub acct86: Account<'info, DataAccount>,
    pub owner16: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_200_program {
    use super::*;

    pub fn case_200(ctx: Context<Case200>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct86.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct86.data = new_balance;
        Ok(())
    }
}
