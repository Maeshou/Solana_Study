// 9. サブスクリプション＋使用履歴
use anchor_lang::prelude::*;
declare_id!("SUBS111122223333444455556666777788");

#[program]
pub mod misinit_subscription_v6 {
    use super::*;

    pub fn init_plan(
        ctx: Context<InitPlan>,
        tier: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        p.tier = tier;
        p.active = true;
        Ok(())
    }

    pub fn upgrade_plan(
        ctx: Context<InitPlan>,
        new_tier: u8,
    ) -> Result<()> {
        let p = &mut ctx.accounts.plan;
        require!(new_tier > p.tier, ErrorCode9::InvalidTier);
        p.tier = new_tier;
        Ok(())
    }

    pub fn record_usage(
        ctx: Context<InitPlan>,
        user: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.usage_log;
        log.users.push(user);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPlan<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1)] pub plan: Account<'info, PlanData>,
    #[account(mut)] pub usage_log: Account<'info, UsageLog6>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct PlanData { pub tier:u8, pub active:bool }
#[account]
pub struct UsageLog6 { pub users: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode9 { #[msg("プラン階層が無効です。現在より上位の階層を指定してください。")] InvalidTier }
