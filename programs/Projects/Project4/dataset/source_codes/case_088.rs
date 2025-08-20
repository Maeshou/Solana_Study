use anchor_lang::prelude::*;

declare_id!("SafeEx34Cooldown11111111111111111111111111");

#[program]
pub mod example34 {
    use super::*;

    pub fn init_cooldown(
        ctx: Context<InitCooldown>,
        actions: u8,
    ) -> Result<()> {
        let c = &mut ctx.accounts.cooldown;
        c.actions        = actions;
        c.cooldowns      = 0;
        c.cooldown_flag  = false;

        // 初期アクション分だけクールダウンを設定
        let mut i = 0u8;
        while i < actions {
            c.cooldowns = c.cooldowns.saturating_add(1);
            i += 1;
        }
        Ok(())
    }

    pub fn trigger_action(
        ctx: Context<TriggerAction>,
    ) -> Result<()> {
        let c = &mut ctx.accounts.cooldown;
        if c.cooldowns < c.actions {
            c.cooldowns = c.cooldowns.saturating_add(1);
            c.cooldown_flag = false;
        } else {
            c.cooldown_flag = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCooldown<'info> {
    #[account(init, payer = user, space = 8 + 1 + 1 + 1)]
    pub cooldown: Account<'info, CooldownData>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TriggerAction<'info> {
    #[account(mut)] pub cooldown: Account<'info, CooldownData>,
}

#[account]
pub struct CooldownData {
    pub actions:        u8,
    pub cooldowns:      u8,
    pub cooldown_flag:  bool,
}
