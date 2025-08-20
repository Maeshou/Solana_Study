use anchor_lang::prelude::*;

declare_id!("55555555555555555555555555555555");

#[program]
pub mod init_settings {
    use super::*;

    pub fn new_settings(
        ctx: Context<NewSettings>,
        threshold: u16,
        limit: u16,
    ) -> Result<()> {
        let settings = &mut ctx.accounts.settings;
        settings.threshold = threshold;
        settings.limit = limit;
        settings.enabled = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NewSettings<'info> {
    #[account(mut)]
    pub settings: Account<'info, SettingsData>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SettingsData {
    pub threshold: u16,
    pub limit: u16,
    pub enabled: bool,
}
