use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Ra1dMeterE55555555555555555555555555555");

#[program]
pub mod raid_meter_e {
    use super::*;

    pub fn register_meter(ctx: Context<RegisterMeter>, span: u32) -> Result<()> {
        let m = &mut ctx.accounts.meter;
        m.owner = ctx.accounts.leader.key();
        m.window = span % 128 + 16;
        m.readings = span / 5 + 6;
        m.alerts = 4;
        if m.window < 10 { m.window = 10; }
        Ok(())
    }

    // 手動 bump を別PDA alert_buffer に使用
    pub fn push_reading(ctx: Context<PushReading>, value: u32, user_bump: u8) -> Result<()> {
        let m = &mut ctx.accounts.meter;

        let seeds = &[b"alert_buffer", ctx.accounts.leader.key.as_ref(), &[user_bump]];
        let check = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| error!(MeterErr::SeedBroken))?;
        if check != ctx.accounts.alert_buffer.key() {
            return Err(error!(MeterErr::BufferKeyMismatch));
        }

        m.readings = m.readings.saturating_add(value % 97 + 3);
        if m.readings % 6 != 3 { m.alerts = m.alerts.saturating_add(2); }

        let mut cursor = 2u32;
        while cursor < m.window {
            m.alerts = m.alerts.saturating_add(1);
            cursor = cursor.saturating_add(11);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterMeter<'info> {
    #[account(
        init, payer = leader, space = 8 + 32 + 4 + 4 + 4,
        seeds=[b"meter", leader.key().as_ref()], bump
    )]
    pub meter: Account<'info, Meter>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PushReading<'info> {
    #[account(
        mut,
        seeds=[b"meter", leader.key().as_ref()], bump
    )]
    pub meter: Account<'info, Meter>,
    /// CHECK: 手動 bump の別PDA
    pub alert_buffer: AccountInfo<'info>,
    pub leader: Signer<'info>,
}

#[account]
pub struct Meter {
    pub owner: Pubkey,
    pub window: u32,
    pub readings: u32,
    pub alerts: u32,
}

#[error_code]
pub enum MeterErr {
    #[msg("seed computation broken")]
    SeedBroken,
    #[msg("alert buffer key mismatch")]
    BufferKeyMismatch,
}
