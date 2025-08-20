use anchor_lang::prelude::*;

declare_id!("VulnInitAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod vuln_iot {
    use super::*;

    pub fn init_device(
        ctx: Context<InitDevice>,
        device_id: String,
        thresholds: Vec<u32>,
    ) -> Result<()> {
        let dev = &mut ctx.accounts.device;           // ← Init OK
        dev.id            = device_id.clone();
        dev.registered_at = Clock::get()?.unix_timestamp;

        // telemetry は init されていない → 任意差し替え可
        let telem = &mut ctx.accounts.telemetry;      // ← Init missing
        telem.values = Vec::new();
        for &t in thresholds.iter() {
            telem.values.push(t);
        }

        let alert = &mut ctx.accounts.alert_cfg;      // ← Init OK
        alert.levels    = thresholds.clone();
        alert.triggered = telem.values.first().map_or(false, |&v| v > thresholds[0]);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDevice<'info> {
    #[account(init, payer = user, space = 8 + 4 + 64 + 8)]
    pub device: Account<'info, DeviceData>,
    pub telemetry: Account<'info, TelemetryData>,  // ← init がない
    #[account(init, payer = user, space = 8 + 4 + (4 * 10) + 1)]
    pub alert_cfg: Account<'info, AlertCfgData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DeviceData {
    pub id: String,
    pub registered_at: i64,
}

#[account]
pub struct TelemetryData {
    pub values: Vec<u32>,
}

#[account]
pub struct AlertCfgData {
    pub levels: Vec<u32>,
    pub triggered: bool,
}
