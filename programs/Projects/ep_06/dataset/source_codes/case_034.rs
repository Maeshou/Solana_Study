use anchor_lang::prelude::*;
declare_id!("OPTI0341111111111111111111111111111111111111");

#[program]
pub mod case034 {
    use super::*;
    pub fn execute_optioncall(ctx: Context<OptionCallContext>) -> Result<()> {
        // Options logic
        let mut opt = OptionAccount::try_from(&ctx.accounts.account_a.to_account_info())?;
        opt.exercised = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OptionCallContext<'info> {
    /// CHECK: expecting OptionCallAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting OptionCallAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct OptionCallAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}