use anchor_lang::prelude::*;
declare_id!("PAYI0771111111111111111111111111111111111111");

#[program]
pub mod case077 {
    use super::*;
    pub fn execute_payinsurance(ctx: Context<PayInsuranceContext>) -> Result<()> {
        // Insurance premium logic
        msg!("Premium calculated");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PayInsuranceContext<'info> {
    /// CHECK: expecting PayInsuranceAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting PayInsuranceAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PayInsuranceAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}