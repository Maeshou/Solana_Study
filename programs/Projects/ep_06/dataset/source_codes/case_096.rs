use anchor_lang::prelude::*;
declare_id!("REGI0961111111111111111111111111111111111111");

#[program]
pub mod case096 {
    use super::*;
    pub fn execute_registerenergygrid(ctx: Context<RegisterEnergyGridContext>) -> Result<()> {
        // IoT or energy logic
        msg!("Grid operation logged");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterEnergyGridContext<'info> {
    /// CHECK: expecting RegisterEnergyGridAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterEnergyGridAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterEnergyGridAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}