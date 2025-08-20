use anchor_lang::prelude::*;
declare_id!("GRAN0591111111111111111111111111111111111111");

#[program]
pub mod case059 {
    use super::*;
    pub fn execute_grantpoints(ctx: Context<GrantPointsContext>) -> Result<()> {
        // Default context logic
        msg!("Case 59 executed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct GrantPointsContext<'info> {
    /// CHECK: expecting GrantPointsAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting GrantPointsAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GrantPointsAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}