use anchor_lang::prelude::*;
declare_id!("REGI0781111111111111111111111111111111111111");

#[program]
pub mod case078 {
    use super::*;
    pub fn execute_registerdigitaltwin(ctx: Context<RegisterDigitalTwinContext>) -> Result<()> {
        // IoT or energy logic
        msg!("Grid operation logged");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDigitalTwinContext<'info> {
    /// CHECK: expecting RegisterDigitalTwinAccount but using UncheckedAccount
    pub account_a: UncheckedAccount<'info>,
    /// CHECK: expecting RegisterDigitalTwinAccount but using UncheckedAccount
    pub account_b: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RegisterDigitalTwinAccount {
    pub dummy: u64,
    pub counter: u64,
    pub version: u8,
}