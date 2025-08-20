// (2) Forge Upgrader — 装備強化（鍛冶台・職人カード・装備盤）
use anchor_lang::prelude::*;
declare_id!("11111111111111111111111111111111");

#[program]
pub mod forge_upgrader {
    use super::*;
    use SmithRole::*;

    pub fn init_workshop(ctx: Context<InitWorkshop>, seed: u64) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        w.owner = ctx.accounts.owner.key();
        w.seed = seed;
        w.tool_count = 0;
        Ok(())
    }

    pub fn enroll_smith(ctx: Context<EnrollSmith>, role: SmithRole, level: u8) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        let s = &mut ctx.accounts.smith;
        s.workshop = w.key();
        s.role = role;
        s.level = level;
        s.reputation = 0;
        w.tool_count = w.tool_count.saturating_add(1);
        Ok(())
    }

    pub fn upgrade_gear(ctx: Context<UpgradeGear>, base_stats: Vec<u16>) -> Result<()> {
        let w = &mut ctx.accounts.workshop;
        let smith = &mut ctx.accounts.smith;
        let inspector = &mut ctx.accounts.inspector;
        let gear = &mut ctx.accounts.gear;
        let panel = &mut ctx.accounts.panel;

        // ループ：平均とマスク
        let mut total: u32 = 0;
        let mut mask: u16 = 0;
        for v in base_stats {
            let capped = v.min(500);
            total = total.saturating_add(capped as u32);
            mask ^= ((capped as u16) << 1) | ((capped as u16) >> 1);
        }
        let avg = if base_stats.is_empty() { 0 } else { (total / base_stats.len() as u32) as u16 };

        // 分岐（4行以上）
        if smith.role == Master {
            gear.power = gear.power.saturating_add(avg as u32 + (mask as u32 & 0x3FF));
            smith.reputation = smith.reputation.saturating_add(2);
            inspector.reputation = inspector.reputation.saturating_add(1);
            w.tool_count = w.tool_count.saturating_add(1);
            msg!("Master upgrade applied: avg={}, mask={}, power={}", avg, mask, gear.power);
        } else {
            gear.power = gear.power.saturating_add(avg as u32 / 2 + ((mask as u32 >> 2) & 0x1FF));
            smith.reputation = smith.reputation.saturating_add(1);
            inspector.reputation = inspector.reputation.saturating_add(2);
            w.tool_count = w.tool_count.saturating_add(1);
            msg!("Apprentice/Journeyman upgrade: avg={}, mask={}, power={}", avg, mask, gear.power);
        }

        // 近似平方根で耐久メーター
        let mut x = (gear.power as u128).max(1);
        let mut i = 0;
        while i < 3 {
            x = (x + (gear.power as u128 / x)).max(1) / 2;
            i += 1;
        }
        panel.workshop = w.key();
        panel.durability_meter = (x as u32).min(1_000_000);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitWorkshop<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 4)]
    pub workshop: Account<'info, Workshop>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnrollSmith<'info> {
    #[account(mut)]
    pub workshop: Account<'info, Workshop>,
    #[account(init, payer = payer, space = 8 + 32 + 1 + 1 + 4)]
    pub smith: Account<'info, SmithCard>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 役割不一致 + 同一親 has_one で同一口座流用を阻止
#[derive(Accounts)]
pub struct UpgradeGear<'info> {
    #[account(mut)]
    pub workshop: Account<'info, Workshop>,
    #[account(
        mut,
        has_one = workshop,
        constraint = smith.role != inspector.role @ ErrCode::CosplayBlocked
    )]
    pub smith: Account<'info, SmithCard>,
    #[account(mut, has_one = workshop)]
    pub inspector: Account<'info, SmithCard>,
    #[account(mut, has_one = workshop)]
    pub gear: Account<'info, Gear>,
    #[account(mut, has_one = workshop)]
    pub panel: Account<'info, ForgePanel>,
}

#[account]
pub struct Workshop {
    pub owner: Pubkey,
    pub seed: u64,
    pub tool_count: u32,
}

#[account]
pub struct SmithCard {
    pub workshop: Pubkey,
    pub role: SmithRole,
    pub level: u8,
    pub reputation: u32,
}

#[account]
pub struct Gear {
    pub workshop: Pubkey,
    pub power: u32,
    pub slot: u8,
}

#[account]
pub struct ForgePanel {
    pub workshop: Pubkey,
    pub durability_meter: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SmithRole {
    Apprentice,
    Journeyman,
    Master,
}

#[error_code]
pub enum ErrCode {
    #[msg("Type cosplay prevented in forge operation.")]
    CosplayBlocked,
}
