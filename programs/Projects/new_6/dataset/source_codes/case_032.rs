// 03. レイド記録：記録アカウントと報酬口座の混用
use anchor_lang::prelude::*;

declare_id!("Ra1dRec0rd333333333333333333333333333333333");

#[program]
pub mod raid_record {
    use super::*;

    pub fn init_raid(ctx: Context<InitRaid>, boss_id: u64) -> Result<()> {
        let r = &mut ctx.accounts.record;
        r.boss_id = boss_id;
        r.damage = 0;
        r.rewarded = false;
        Ok(())
    }

    pub fn act_submit(ctx: Context<SubmitRaid>, dmg: u64) -> Result<()> {
        let r = &mut ctx.accounts.record;
        let reward = &mut ctx.accounts.output;

        if dmg > 0 {
            r.damage = r.damage.saturating_add(dmg);
        }

        if r.damage > 5000 {
            r.rewarded = true;
        }

        reward.boss_id = r.boss_id;
        reward.rewarded = r.rewarded;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRaid<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 1)]
    pub record: Account<'info, Raid>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitRaid<'info> {
    #[account(mut)]
    pub record: Account<'info, Raid>,
    #[account(mut)]
    pub output: Account<'info, Raid>, // Type Cosplay: 報酬と記録が同一構造体
    pub reporter: AccountInfo<'info>,
}

#[account]
pub struct Raid {
    pub boss_id: u64,
    pub damage: u64,
    pub rewarded: bool,
}
