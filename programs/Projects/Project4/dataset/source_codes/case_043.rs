use anchor_lang::prelude::*;

declare_id!("InitAllAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

#[program]
pub mod multi_init10 {
    use super::*;

    // デバイス情報、ティアラリー、アラート条件を初期化
    pub fn init_device(
        ctx: Context<InitDevice>,
        device_id: String,
        thresholds: Vec<u32>,
    ) -> Result<()> {
        let dev = &mut ctx.accounts.device;
        dev.id = device_id.clone();
        dev.registered_at = Clock::get()?.unix_timestamp;

        let telem = &mut ctx.accounts.telemetry;
        telem.values = Vec::new();

        let alert = &mut ctx.accounts.alert_cfg;
        alert.levels = thresholds.clone();
        alert.triggered = false;

        // テレメトリの初期ダミー値を thresholds.length 回だけ push
        for &t in thresholds.iter() {
            telem.values.push(t);
        }

        // 最初の閾値超過チェック
        if let Some(&first) = telem.values.first() {
            if first > thresholds[0] {
                alert.triggered = true;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDevice<'info> {
    #[account(init, payer = user, space = 8 + 4 + 64 + 8)]
    pub device: Account<'info, DeviceData>,
    #[account(init, payer = user, space = 8 + 4 + (8 * 20))]
    pub telemetry: Account<'info, TelemetryData>,
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
