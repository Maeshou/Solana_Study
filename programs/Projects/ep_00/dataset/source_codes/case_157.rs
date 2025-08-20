use anchor_lang::prelude::*;

declare_id!("TjhQGPFPKVc3hFCcCi6gXMZUhuCaigkKwGdka6GdMvAK");

#[derive(Accounts)]
pub struct Case157<'info> {
    #[account(mut, has_one = owner39)] pub acct3: Account<'info, DataAccount>,
    pub owner39: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_157_program {
    use super::*;

    pub fn case_157(ctx: Context<Case157>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct3.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct3.data = new_balance;
        Ok(())
    }
}
