// 02. 装備強化：装備／検証ログに同じアカウント使える脆弱構造
use anchor_lang::prelude::*;

declare_id!("EqUpgRaDe222222222222222222222222222222222");

#[program]
pub mod equipment_upgrade {
    use super::*;

    pub fn init_equipment(ctx: Context<InitEq>, kind: u8) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        eq.kind = kind;
        eq.level = 1;
        eq.active = true;
        Ok(())
    }

    pub fn act_upgrade(ctx: Context<Upgrade>, effort: u32) -> Result<()> {
        let eq = &mut ctx.accounts.equipment;
        let log = &mut ctx.accounts.record;

        for _ in 0..effort {
            if eq.level < 50 {
                eq.level += 1;
            }
        }

        if eq.level % 10 == 0 {
            eq.active = !eq.active;
        }

        log.level = eq.level;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEq<'info> {
    #[account(init, payer = user, space = 8 + 1 + 4 + 1 + 4)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Upgrade<'info> {
    #[account(mut)]
    pub equipment: Account<'info, Equipment>,
    #[account(mut)]
    pub record: Account<'info, Equipment>, // Type Cosplay: 装備とログを同一構造体
    pub handler: AccountInfo<'info>,
}

#[account]
pub struct Equipment {
    pub kind: u8,
    pub level: u32,
    pub active: bool,
    pub padding: u32, // unused
}
