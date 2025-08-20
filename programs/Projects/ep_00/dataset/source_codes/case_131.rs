use anchor_lang::prelude::*;

declare_id!("KZyHqmIoyM87FK1Qga2Twaxxrf65fnlx1trxrkPc8Kk2");

#[derive(Accounts)]
pub struct Case131<'info> {
    #[account(mut, has_one = owner19)] pub acct50: Account<'info, DataAccount>,
    #[account(mut)] pub acct41: Account<'info, DataAccount>,
    #[account(mut)] pub acct26: Account<'info, DataAccount>,
    pub owner19: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_131_program {
    use super::*;

    pub fn case_131(ctx: Context<Case131>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct50.data = set_val;
        Ok(())
    }
}
