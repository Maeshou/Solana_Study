use anchor_lang::prelude::*;

declare_id!("ReSoUrCeRaId111111111111111111111111111");

#[program]
pub mod resource_and_raid_engine {
    use super::*;

    pub fn init_resource_nodes(
        ctx: Context<InitResourceNodes>,
        tier: TerritoryTier,
        baseline: u32,
    ) -> Result<()> {
        let nodes = &mut ctx.accounts.resource_hub;
        let now = Clock::get()?.unix_timestamp;

        nodes.owner = ctx.accounts.owner.key();
        nodes.created_at = now;
        nodes.tier = tier.clone();

        // 5種類を生成。順序固定を避けるため、start偏移を使う
        let mut list: Vec<ResourceNode> = Vec::new();
        let start = (now as u64).rotate_left(5) as usize % 5;
        let mut i = 0usize;
        while i < 5 {
            let index = (start + i) % 5;
            let rtype = to_resource_type(index);

            // ティア別の上書き（段階的）
            let mut gen = baseline.max(10);
            if matches_city(&tier) { gen = gen.saturating_add(20 + (index as u32 * 5)); }
            if matches_capital(&tier) { gen = gen.saturating_add(50 + (index as u32 * 10)); }
            if matches_outpost(&tier) { gen = gen.saturating_add(10 + (index as u32 * 3)); }

            let cap = gen.saturating_mul(24);
            list.push(ResourceNode {
                resource_type: rtype,
                current_amount: 0,
                generation_rate: gen,
                capacity: cap,
                last_harvest: now,
                enhancement_level: 0,
            });
            i = i.saturating_add(1);
        }

        nodes.nodes = list;
        Ok(())
    }

    pub fn harvest_and_raid(
        ctx: Context<HarvestAndRaid>,
        party_size: u8,
        difficulty: RaidDifficulty,
        hours_elapsed: u32,
    ) -> Result<()> {
        let hub = &mut ctx.accounts.resource_hub;
        let now = Clock::get()?.unix_timestamp;

        require!(party_size >= 1, ErrorCode::PartyTooSmall);
        require!(party_size <= 6, ErrorCode::PartyTooLarge);
        require!(hours_elapsed > 0, ErrorCode::InvalidHours);

        // 収穫：各ノードに対して、開始点をずらしてループ（順序固定回避）
        let start = ((now as u64) ^ (party_size as u64)).rotate_left(3) as usize % hub.nodes.len().max(1);
        let mut j = 0usize;
        while j < hub.nodes.len() {
            let k = (start + j) % hub.nodes.len();
            let rate = hub.nodes[k].generation_rate;
            let add = rate.saturating_mul(hours_elapsed);
            let new_amt = hub.nodes[k].current_amount.saturating_add(add);
            let capped = new_amt.min(hub.nodes[k].capacity);
            hub.nodes[k].current_amount = capped;
            hub.nodes[k].last_harvest = now;
            j = j.saturating_add(1);
        }

        // レイドのエンカウント強度：難易度・パーティサイズ・時刻由来で段階的に上書き
        let mut threat = 50u32;
        if is_normal(&difficulty) { threat = threat.saturating_add(30); }
        if is_hard(&difficulty) { threat = threat.saturating_add(60); }
        if is_mythic(&difficulty) { threat = threat.saturating_add(120); }
        if party_size >= 4 { threat = threat.saturating_add(25); }
        if now % 2 == 0 { threat = threat.saturating_add(7); }

        // ドロップ判定はseed回転で実行順を変化
        let mut drops: Vec<Loot> = Vec::new();
        let seed = ((now as u64).rotate_left(9)) ^ (threat as u64);
        let mut turn = 0u8;
        while (turn as usize) < hub.nodes.len() {
            let bias = ((seed >> (turn % 13)) as u32) % 100;
            let rarity = decide_rarity(threat, bias);
            let item = make_item(threat, rarity, turn);
            drops.push(item);
            turn = turn.saturating_add(1);
        }

        // ログ的に保持（最大N件・古いものから削る）
        push_drops(&mut ctx.accounts.raid_log, &drops);

        Ok(())
    }

    fn to_resource_type(i: usize) -> ResourceType {
        // match禁止なのでif多段
        let mut t = ResourceType::Gold;
        if i == 1 { t = ResourceType::Crystal; }
        if i == 2 { t = ResourceType::Wood; }
        if i == 3 { t = ResourceType::Stone; }
        if i >= 4 { t = ResourceType::MagicEssence; }
        t
    }

    fn matches_city(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::City) }
    fn matches_capital(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::Capital) }
    fn matches_outpost(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::Outpost) }

    fn is_normal(d: &RaidDifficulty) -> bool { matches!(d, RaidDifficulty::Normal) }
    fn is_hard(d: &RaidDifficulty) -> bool { matches!(d, RaidDifficulty::Hard) }
    fn is_mythic(d: &RaidDifficulty) -> bool { matches!(d, RaidDifficulty::Mythic) }

    fn decide_rarity(threat: u32, bias: u32) -> Rarity {
        let mut score = threat.saturating_add(bias);
        if score >= 180 { return Rarity::Mythic; }
        if score >= 120 { return Rarity::Legendary; }
        if score >= 80 { return Rarity::Epic; }
        if score >= 40 { return Rarity::Rare; }
        Rarity::Common
    }

    fn make_item(threat: u32, rarity: Rarity, turn: u8) -> Loot {
        // レア度で段階的に上書き：ステータスは最終的に加算で決まる
        let mut p = 10u32;
        let mut d = 10u32;
        let mut u = 1u32;

        if matches!(rarity, Rarity::Rare)     { p = p.saturating_add(20); d = d.saturating_add(15); u = u.saturating_add(1); }
        if matches!(rarity, Rarity::Epic)     { p = p.saturating_add(40); d = d.saturating_add(35); u = u.saturating_add(2); }
        if matches!(rarity, Rarity::Legendary){ p = p.saturating_add(70); d = d.saturating_add(60); u = u.saturating_add(3); }
        if matches!(rarity, Rarity::Mythic)   { p = p.saturating_add(120); d = d.saturating_add(110); u = u.saturating_add(4); }

        // threat と turn で微調整
        let spice = ((threat.rotate_left((turn % 11) as u32)) % 23) as u32;
        Loot {
            rarity,
            power: p.saturating_add(spice),
            defense: d.saturating_add(spice / 2),
            utility: u.saturating_add(spice % 3),
        }
    }

    fn push_drops(log: &mut Account<RaidLog>, new_items: &Vec<Loot>) {
        let mut i = 0usize;
        while i < new_items.len() {
            log.recent.push(new_items[i].clone());
            i = i.saturating_add(1);
        }
        // 上限管理（古い方を削る）
        while log.recent.len() > 16 {
            log.recent.remove(0);
        }
    }
}

#[derive(Accounts)]
pub struct InitResourceNodes<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + ResourceHub::MAX_SPACE,
        seeds = [b"resource-hub", owner.key().as_ref()],
        bump
    )]
    pub resource_hub: Account<'info, ResourceHub>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct HarvestAndRaid<'info> {
    #[account(mut)]
    pub resource_hub: Account<'info, ResourceHub>,
    #[account(
        init_if_needed,
        payer = executor,
        space = 8 + RaidLog::MAX_SPACE,
        seeds = [b"raid-log", executor.key().as_ref()],
        bump
    )]
    pub raid_log: Account<'info, RaidLog>,
    #[account(mut)]
    pub executor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ResourceHub {
    pub owner: Pubkey,
    pub created_at: i64,
    pub tier: TerritoryTier,
    pub nodes: Vec<ResourceNode>,
}
impl ResourceHub { pub const MAX_SPACE: usize = 32 + 8 + 1 + (4 + 5 * ResourceNode::SIZE); }

#[account]
pub struct RaidLog { pub recent: Vec<Loot> }
impl RaidLog { pub const MAX_SPACE: usize = 4 + 16 * Loot::SIZE; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub current_amount: u32,
    pub generation_rate: u32,
    pub capacity: u32,
    pub last_harvest: i64,
    pub enhancement_level: u8,
}
impl ResourceNode { pub const SIZE: usize = 1 + 4 + 4 + 4 + 8 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier { Outpost, City, Capital }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ResourceType { Gold, Crystal, Wood, Stone, MagicEssence }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RaidDifficulty { Normal, Hard, Mythic }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Rarity { Common, Rare, Epic, Legendary, Mythic }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Loot {
    pub rarity: Rarity,
    pub power: u32,
    pub defense: u32,
    pub utility: u32,
}
impl Loot { pub const SIZE: usize = 1 + 4 + 4 + 4; }

#[error_code]
pub enum ErrorCode {
    #[msg("Party size too small")] PartyTooSmall,
    #[msg("Party size too large")] PartyTooLarge,
    #[msg("Elapsed hours must be positive")] InvalidHours,
}
