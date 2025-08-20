// 5. 機能フラグ＋使用履歴
use anchor_lang::prelude::*;
declare_id!("FEAT111122223333444455556666777788");

#[program]
pub mod misinit_feature_v6 {
    use super::*;

    pub fn init_feature(
        ctx: Context<InitFeat>,
        feature_name: String,
    ) -> Result<()> {
        let f = &mut ctx.accounts.feature;
        f.name = feature_name;
        f.enabled = false;
        Ok(())
    }

    pub fn toggle_feature(ctx: Context<InitFeat>) -> Result<()> {
        let f = &mut ctx.accounts.feature;
        f.enabled = !f.enabled;
        Ok(())
    }

    pub fn record_usage(
        ctx: Context<InitFeat>,
        user: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.usage_log;
        log.users.push(user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitFeat<'info> {
    #[account(init, payer = user, space = 8 + (4+32) + 1)] pub feature: Account<'info, Feat>,
    #[account(mut)] pub usage_log: Account<'info, UsageLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct Feat { pub name: String, pub enabled: bool }
#[account]
pub struct UsageLog { pub users: Vec<Pubkey> }