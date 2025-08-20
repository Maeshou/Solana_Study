// 4. 機能トグル＋監査ログ
use anchor_lang::prelude::*;
declare_id!("TOGL111122223333444455556666777788");

#[program]
pub mod misinit_toggle_v7 {
    use super::*;

    pub fn init_toggle(
        ctx: Context<InitToggle>,
        feature: String,
    ) -> Result<()> {
        let t = &mut ctx.accounts.toggle;
        t.feature = feature;
        t.enabled = false;
        Ok(())
    }

    pub fn toggle_feature(ctx: Context<InitToggle>) -> Result<()> {
        let t = &mut ctx.accounts.toggle;
        t.enabled = !t.enabled;
        Ok(())
    }

    pub fn log_toggle(
        ctx: Context<InitToggle>,
        user: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.toggle_log;
        log.users.push(user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitToggle<'info> {
    #[account(init, payer = admin, space = 8 + (4+32) + 1)] pub toggle: Account<'info, ToggleData>,
    #[account(mut)] pub toggle_log: Account<'info, ToggleLog>,
    #[account(mut)] pub admin: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct ToggleData { pub feature: String, pub enabled: bool }
#[account]
pub struct ToggleLog { pub users: Vec<Pubkey> }