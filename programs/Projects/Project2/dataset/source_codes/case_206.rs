use anchor_lang::prelude::*;

declare_id!("Hlth22222222222222222222222222222222222");

#[program]
pub mod health_pool {
    use super::*;

    pub fn init(ctx: Context<InitHealth>, max_hp: u64) -> Result<()> {
        let h = &mut ctx.accounts.health;
        h.max_hp = max_hp;
        h.current_hp = max_hp;
        h.regen_rate = 5;    // 固定再生レート
        Ok(())
    }

    pub fn apply_damage(ctx: Context<ModifyHealth>, dmg: u64) -> Result<()> {
        let h = &mut ctx.accounts.health;
        h.current_hp = h.current_hp.saturating_sub(dmg);
        Ok(())
    }

    pub fn apply_heal(ctx: Context<ModifyHealth>, heal: u64) -> Result<()> {
        let h = &mut ctx.accounts.health;
        h.current_hp = (h.current_hp + heal).min(h.max_hp);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHealth<'info> {
    #[account(init, payer = user, space = 8 + 8*3 + 1)]
    pub health: Account<'info, HealthData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyHealth<'info> {
    #[account(mut)] pub health: Account<'info, HealthData>,
}

#[account]
pub struct HealthData {
    pub max_hp: u64,
    pub current_hp: u64,
    pub regen_rate: u8,
    pub reserved: u8, // アライン用
}
