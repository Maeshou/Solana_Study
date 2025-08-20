use anchor_lang::prelude::*;

declare_id!("fHToHJmhDPSkC2oNAJpI72MGpbn1c3H9IXIGXYTZ56ve");

#[derive(Accounts)]
pub struct Case114<'info> {
    #[account(mut, has_one = owner19)] pub acct23: Account<'info, DataAccount>,
    #[account(mut)] pub acct17: Account<'info, DataAccount>,
    #[account(mut)] pub acct99: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_114_program {
    use super::*;

    pub fn case_114(ctx: Context<Case114>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let balance = ctx.accounts.acct23.data;
        let new_balance = balance.checked_add(amount).unwrap();
        ctx.accounts.acct23.data = new_balance;
        Ok(())
    }
}
