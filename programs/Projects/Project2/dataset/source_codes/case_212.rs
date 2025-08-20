use anchor_lang::prelude::*;

declare_id!("DurExa1111111111111111111111111111111111");

#[program]
pub mod durable_extra {
    use super::*;

    pub fn init(ctx: Context<InitDur>, max: u8) -> Result<()> {
        let d = &mut ctx.accounts.dur;
        d.max = max;
        d.current = max;
        d.repairs_required = 0;
        Ok(())
    }

    pub fn use_item(ctx: Context<UseDur>, amount: u8) -> Result<()> {
        let d = &mut ctx.accounts.dur;
        if d.current >= amount {
            // 通常消費
            d.current = d.current.saturating_sub(amount);
            // 使用回数を記録
            d.times_used = d.times_used.saturating_add(1);
        } else {
            // 耐久不足による故障扱い
            d.broken = true;
            // 修理要請をカウント
            d.repairs_required = d.repairs_required.saturating_add(1);
            // 代替ダメージを蓄積
            d.over_damage = d.over_damage.saturating_add(amount - d.current);
            d.current = 0;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDur<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 8 + 1 + 8)]
    pub dur: Account<'info, DurData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UseDur<'info> {
    #[account(mut)] pub dur: Account<'info, DurData>,
    pub user: Signer<'info>,
}

#[account]
pub struct DurData {
    pub max: u8,
    pub current: u8,
    pub times_used: u64,
    pub broken: bool,
    pub repairs_required: u64,
    pub over_damage: u8,
}
