use anchor_lang::prelude::*;

declare_id!("DeFeNsEFoRgE11111111111111111111111111");

#[program]
pub mod defense_layout_forge {
    use super::*;

    pub fn init_defense_layout(
        ctx: Context<InitDefenseLayout>,
        size_hint: u32,
        tier: TerritoryTier,
        order_seed: u64,
    ) -> Result<()> {
        let layout = &mut ctx.accounts.defense_plan;
        let now = Clock::get()?.unix_timestamp;

        require!(size_hint >= 100, ErrorCode::TooSmallHint);

        layout.created_at = now;
        layout.tier = tier.clone();
        layout.estimated_size = size_hint;

        let count = slot_count_from_size(size_hint);
        layout.slot_count = count;

        let base = base_stats_for_tier(&tier);
        layout.structures = build_structures(count, order_seed, base, now);

        Ok(())
    }

    fn slot_count_from_size(size_hint: u32) -> u32 {
        let mut c = 3u32;
        if size_hint >= 2_000 { c = 5; }
        if size_hint >= 5_000 { c = 8; }
        c
    }

    #[derive(Clone)]
    struct BaseStats { hp: u32, dp: u32, upkeep: u32 }

    fn base_stats_for_tier(t: &TerritoryTier) -> BaseStats {
        let mut b = BaseStats { hp: 900, dp: 120, upkeep: 18 };
        if matches!(t, TerritoryTier::City) {
            b.hp = b.hp.saturating_add(400);
            b.dp = b.dp.saturating_add(60);
            b.upkeep = b.upkeep.saturating_add(10);
        }
        if matches!(t, TerritoryTier::Capital) {
            b.hp = b.hp.saturating_add(900);
            b.dp = b.dp.saturating_add(180);
            b.upkeep = b.upkeep.saturating_add(30);
        }
        b
    }

    fn build_structures(count: u32, seed: u64, base: BaseStats, now: i64) -> Vec<DefenseStructure> {
        let mut out = Vec::new();
        let start = (seed as usize) % (count as usize);
        let mut i = 0usize;
        while i < count as usize {
            let idx = (start + i) % (count as usize);
            let kind = choose_kind(idx);
            let tweak = ((seed.rotate_left((idx as u32) & 13)) as u32) % 37;
            let hp = base.hp.saturating_add(tweak * 10);
            let dp = base.dp.saturating_add((tweak % 9) * 10);
            let up = base.upkeep.saturating_add(tweak % 7);

            out.push(DefenseStructure {
                structure_id: idx as u32,
                structure_type: kind,
                health_points: hp,
                defense_power: dp,
                maintenance_cost: up,
                last_upgrade: now,
            });
            i = i.saturating_add(1);
        }
        out
    }

    fn choose_kind(idx: usize) -> StructureType {
        let mut k = StructureType::Watchtower;
        if idx % 4 == 1 { k = StructureType::BarricadeWall; }
        if idx % 4 == 2 { k = StructureType::MagicBarrier; }
        if idx % 4 >= 3 { k = StructureType::TrapField; }
        k
    }
}

#[derive(Accounts)]
pub struct InitDefenseLayout<'info> {
    #[account(
        init,
        payer = planner,
        space = 8 + DefensePlan::MAX_SPACE,
        seeds = [b"defense-plan", planner.key().as_ref()],
        bump
    )]
    pub defense_plan: Account<'info, DefensePlan>,
    #[account(mut)]
    pub planner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DefensePlan {
    pub created_at: i64,
    pub tier: TerritoryTier,
    pub estimated_size: u32,
    pub slot_count: u32,
    pub structures: Vec<DefenseStructure>,
}
impl DefensePlan { pub const MAX_SPACE: usize = 8 + 1 + 4 + 4 + (4 + 5 * DefenseStructure::SIZE); }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DefenseStructure {
    pub structure_id: u32,
    pub structure_type: StructureType,
    pub health_points: u32,
    pub defense_power: u32,
    pub maintenance_cost: u32,
    pub last_upgrade: i64,
}
impl DefenseStructure { pub const SIZE: usize = 4 + 1 + 4 + 4 + 4 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier { Outpost, City, Capital }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum StructureType { Watchtower, BarricadeWall, MagicBarrier, TrapField }

#[error_code]
pub enum ErrorCode { #[msg("Size hint too small")] TooSmallHint }
