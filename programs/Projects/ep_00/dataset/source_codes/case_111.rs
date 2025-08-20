use anchor_lang::prelude::*;

declare_id!("siJK5gcfcJLFGqa53modskoTVMxikMYw7t35fdUL5NZy");

#[derive(Accounts)]
pub struct Case111<'info> {
    #[account(mut, has_one = owner28)] pub acct6: Account<'info, DataAccount>,
    #[account(mut)] pub acct81: Account<'info, DataAccount>,
    #[account(mut)] pub acct11: Account<'info, DataAccount>,
    pub owner28: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_111_program {
    use super::*;

    pub fn case_111(ctx: Context<Case111>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct6.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct6.data = new_balance;
        Ok(())
    }
}
