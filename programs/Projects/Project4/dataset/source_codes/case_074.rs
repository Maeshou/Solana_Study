use anchor_lang::prelude::*;

declare_id!("SafeEx20Prefs111111111111111111111111111111");

#[program]
pub mod example20 {
    use super::*;

    pub fn init_prefs(
        ctx: Context<InitPrefs>,
        volume: u8,
        brightness: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.prefs;
        p.volume     = volume;
        p.brightness = brightness;
        p.mode_flag  = false;

        // 音量と明るさの合計でモード
        let sum = p.volume as u16 + p.brightness as u16;
        if sum > 200 {
            p.mode_flag = true;
        }
        Ok(())
    }

    pub fn adjust_prefs(
        ctx: Context<AdjustPrefs>,
        vol_delta: i8,
        bri_delta: i8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.prefs;
        // 音量調整
        if vol_delta >= 0 {
            p.volume = p.volume.saturating_add(vol_delta as u8);
        } else {
            p.volume = p.volume.saturating_sub((-vol_delta) as u8);
        }
        // 明るさ調整
        if bri_delta >= 0 {
            p.brightness = p.brightness.saturating_add(bri_delta as u8);
        } else {
            p.brightness = p.brightness.saturating_sub((-bri_delta) as u8);
        }

        // 合計再判定
        let sum = p.volume as u16 + p.brightness as u16;
        if sum > 180 {
            p.mode_flag = true;
        } else {
            p.mode_flag = false;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPrefs<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub prefs: Account<'info, PreferencesData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdjustPrefs<'info> {
    #[account(mut)] pub prefs: Account<'info, PreferencesData>,
}

#[account]
pub struct PreferencesData {
    pub volume:     u8,
    pub brightness: u8,
    pub mode_flag:  bool,
}
