use anchor_lang::prelude::*;

declare_id!("XKUYC1S4RdDuvpBv4YpQrtAVLDKLfrHqjbUVIkL0LEjr");

#[derive(Accounts)]
pub struct Case126<'info> {
    #[account(mut, has_one = owner20)] pub acct99: Account<'info, DataAccount>,
    pub owner20: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_126_program {
    use super::*;

    pub fn case_126(ctx: Context<Case126>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let sub_val = ctx.accounts.acct99.data;
        let result = sub_val.saturating_sub(amount.checked_div(2).unwrap());
        ctx.accounts.acct99.data = result;
        Ok(())
    }
}
