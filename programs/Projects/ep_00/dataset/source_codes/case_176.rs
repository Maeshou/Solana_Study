use anchor_lang::prelude::*;

declare_id!("hDai2dNySeAQN68Sl4bIem9awR1mZMVoNwHXjVcFLKYd");

#[derive(Accounts)]
pub struct Case176<'info> {
    #[account(mut, has_one = owner24)] pub acct20: Account<'info, DataAccount>,
    pub owner24: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_176_program {
    use super::*;

    pub fn case_176(ctx: Context<Case176>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct20.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct20.data = result;
        Ok(())
    }
}
