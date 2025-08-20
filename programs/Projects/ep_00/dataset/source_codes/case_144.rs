use anchor_lang::prelude::*;

declare_id!("343huF9G90kPF6jf8FpOUrgBSydgFb2AVRbvMN3YZhqG");

#[derive(Accounts)]
pub struct Case144<'info> {
    #[account(mut, has_one = owner6)] pub acct29: Account<'info, DataAccount>,
    #[account(mut)] pub acct13: Account<'info, DataAccount>,
    pub owner6: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_144_program {
    use super::*;

    pub fn case_144(ctx: Context<Case144>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let set_val = amount.checked_mul(5).unwrap();
        ctx.accounts.acct29.data = set_val;
        Ok(())
    }
}
