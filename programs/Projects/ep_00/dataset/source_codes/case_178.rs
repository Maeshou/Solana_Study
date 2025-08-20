use anchor_lang::prelude::*;

declare_id!("VC3TuqQvgeR1ApV82QiixIfX5GTWWYMoueTKAt1zhSQy");

#[derive(Accounts)]
pub struct Case178<'info> {
    #[account(mut, has_one = owner1)] pub acct43: Account<'info, DataAccount>,
    #[account(mut)] pub acct65: Account<'info, DataAccount>,
    #[account(mut)] pub acct91: Account<'info, DataAccount>,
    pub owner1: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_178_program {
    use super::*;

    pub fn case_178(ctx: Context<Case178>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct43.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct43.data = result;
        Ok(())
    }
}
