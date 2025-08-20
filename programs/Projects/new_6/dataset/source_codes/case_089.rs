use anchor_lang::prelude::*;

declare_id!("EnergyForge555555555555555555555555555555555");

#[program]
pub mod energy_forge {
    use super::*;

    pub fn activate_reactor(ctx: Context<ActivateReactor>, override_code: u32) -> Result<()> {
        let forge = &mut ctx.accounts.device;
        let operator = &ctx.accounts.controller;
        let diagnostics = &mut ctx.accounts.device_log;

        if override_code == 777_777 {
            diagnostics.data.borrow_mut()[0] = 0x42;
            forge.failure_count = 0;
        } else {
            diagnostics.data.borrow_mut()[1] = 0x13;
            forge.failure_count += 1;
        }

        for i in 0..8 {
            diagnostics.data.borrow_mut()[i + 2] = i as u8 ^ override_code as u8;
        }

        if forge.failure_count > 3 {
            forge.auto_shutdown = true;
            diagnostics.data.borrow_mut()[15] = 0xFF;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ActivateReactor<'info> {
    #[account(mut)]
    pub device: AccountInfo<'info>, // Could be any machine account, misused
    #[account(mut)]
    pub controller: AccountInfo<'info>, // Type Cosplay: could be mechanic or admin
    #[account(mut)]
    pub device_log: AccountInfo<'info>,
}
