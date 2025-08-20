use anchor_lang::prelude::*;

declare_id!("nsuBKxWtraJJeJugWUMdPRreGHshOsNmC6CxOHerwWPv");

#[derive(Accounts)]
pub struct Case146<'info> {
    #[account(mut, has_one = owner14)] pub acct17: Account<'info, DataAccount>,
    pub owner14: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
    pub owner: Pubkey,
}

#[program]
pub mod case_146_program {
    use super::*;

    pub fn case_146(ctx: Context<Case146>, amount: u64) -> Result<()> {
        // Safe code with owner check enforced by has_one
        let original = ctx.accounts.acct17.data;
        let tripled = original.checked_mul(3).unwrap();
        ctx.accounts.acct17.data = tripled;
        Ok(())
    }
}
