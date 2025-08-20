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

        // 構築数は段階的上書き
        let mut count = 3u32;
        if size_hint >= 2_000 { count = 5; }
        if size_hint >= 5_000 { count = 8; }
        layout.slot_count = count;

        // seedで並び順の開始点を回転（固定順序を避ける）
        let start = (order_seed as usize) % (count as usize);
        let mut built: Vec<DefenseStructure> = Vec::new();

        // ティアに応じた基準値をまず設定し、後で個別上書き
        let mut hp_base = 900u32;
        let mut def_base = 120u32;
        let mut upkeep = 18u32;

        if matches_city(&tier) {
            hp_base = hp_base.saturating_add(400);
            def_base = def_base.saturating_add(60);
            upkeep = upkeep.saturating_add(10);
        }
        if matches_capital(&tier) {
            hp_base = hp_base.saturating_add(900);
            def_base = def_base.saturating_add(180);
            upkeep = upkeep.saturating_add(30);
        }

        // 回転開始点をずらして生成
        let mut index = 0usize;
        while index < count as usize {
            let idx = (start + index) % (count as usize);

            // 種類は if の多段上書き（match禁止）
            let mut kind = StructureType::Watchtower;
            if idx % 4 == 1 { kind = StructureType::BarricadeWall; }
            if idx % 4 == 2 { kind = StructureType::MagicBarrier; }
            if idx % 4 >= 3 { kind = StructureType::TrapField; }

            // 個別補正：idxとseedから微調整
            let tweak = ((order_seed.rotate_left((idx as u32) & 13)) as u32) % 37;
            let hp = hp_base.saturating_add(tweak * 10);
            let dp = def_base.saturating_add((tweak % 9) * 10);
            let m = upkeep.saturating_add(tweak % 7);

            built.push(DefenseStructure {
                structure_id: idx as u32,
                structure_type: kind,
                health_points: hp,
                defense_power: dp,
                maintenance_cost: m,
                last_upgrade: now,
            });
            index = index.saturating_add(1);
        }

        layout.structures = built;
        Ok(())
    }

    fn matches_city(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::City) }
    fn matches_capital(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::Capital) }
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
impl DefensePlan {
    pub const MAX_SPACE: usize = 8 + 1 + 4 + 4 + (4 + 5 * DefenseStructure::SIZE);
}

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
pub enum ErrorCode {
    #[msg("Size hint too small")] TooSmallHint,
}
