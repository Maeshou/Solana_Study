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
        nodes.nodes = build_nodes(now, tier, baseline);
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

        harvest_nodes(hub, now, party_size, hours_elapsed);

        let threat = raid_threat(now, party_size, &difficulty);
        let drops = roll_loot(now, hub.nodes.len(), threat);

        push_drops(&mut ctx.accounts.raid_log, &drops);
        Ok(())
    }

    fn build_nodes(now: i64, tier: TerritoryTier, baseline: u32) -> Vec<ResourceNode> {
        let mut out = Vec::new();
        let start = (now as u64).rotate_left(5) as usize % 5;
        let mut i = 0usize;
        while i < 5 {
            let index = (start + i) % 5;
            let kind = resource_kind(index);
            let gen = tiered_generation(baseline.max(10), &tier, index as u32);
            let cap = gen.saturating_mul(24);
            out.push(ResourceNode {
                resource_type: kind, current_amount: 0, generation_rate: gen,
                capacity: cap, last_harvest: now, enhancement_level: 0,
            });
            i = i.saturating_add(1);
        }
        out
    }

    fn resource_kind(i: usize) -> ResourceType {
        let mut t = ResourceType::Gold;
        if i == 1 { t = ResourceType::Crystal; }
        if i == 2 { t = ResourceType::Wood; }
        if i == 3 { t = ResourceType::Stone; }
        if i >= 4 { t = ResourceType::MagicEssence; }
        t
    }

    fn tiered_generation(mut base: u32, tier: &TerritoryTier, idx: u32) -> u32 {
        if matches!(tier, TerritoryTier::City) { base = base.saturating_add(20 + idx * 5); }
        if matches!(tier, TerritoryTier::Capital) { base = base.saturating_add(50 + idx * 10); }
        if matches!(tier, TerritoryTier::Outpost) { base = base.saturating_add(10 + idx * 3); }
        base
    }

    fn harvest_nodes(hub: &mut Account<ResourceHub>, now: i64, party_size: u8, hours: u32) {
        let len = hub.nodes.len().max(1);
        let start = ((now as u64) ^ (party_size as u64)).rotate_left(3) as usize % len;
        let mut j = 0usize;
        while j < hub.nodes.len() {
            let k = (start + j) % hub.nodes.len();
            let add = hub.nodes[k].generation_rate.saturating_mul(hours);
            let new_amt = hub.nodes[k].current_amount.saturating_add(add);
            hub.nodes[k].current_amount = new_amt.min(hub.nodes[k].capacity);
            hub.nodes[k].last_harvest = now;
            j = j.saturating_add(1);
        }
    }

    fn raid_threat(now: i64, party_size: u8, difficulty: &RaidDifficulty) -> u32 {
        let mut t = 50u32;
        t = add_threat_normal(t, difficulty);
        t = add_threat_hard(t, difficulty);
        t = add_threat_mythic(t, difficulty);
        if party_size >= 4 { t = t.saturating_add(25); }
        if now % 2 == 0 { t = t.saturating_add(7); }
        t
    }
    fn add_threat_normal(mut t: u32, d: &RaidDifficulty) -> u32 { if matches!(d, RaidDifficulty::Normal) { t = t.saturating_add(30); } t }
    fn add_threat_hard(mut t: u32, d: &RaidDifficulty) -> u32 { if matches!(d, RaidDifficulty::Hard) { t = t.saturating_add(60); } t }
    fn add_threat_mythic(mut t: u32, d: &RaidDifficulty) -> u32 { if matches!(d, RaidDifficulty::Mythic) { t = t.saturating_add(120); } t }

    fn roll_loot(now: i64, node_len: usize, threat: u32) -> Vec<Loot> {
        let mut out = Vec::new();
        let seed = ((now as u64).rotate_left(9)) ^ (threat as u64);
        let mut turn = 0u8;
        while (turn as usize) < node_len {
            let bias = ((seed >> (turn % 13)) as u32) % 100;
            let r = rarity_from_score(threat, bias);
            out.push(make_item(threat, r, turn));
            turn = turn.saturating_add(1);
        }
        out
    }

    fn rarity_from_score(threat: u32, bias: u32) -> Rarity {
        let s = threat.saturating_add(bias);
        let mut r = Rarity::Common;
        if s >= 40 { r = Rarity::Rare; }
        if s >= 80 { r = Rarity::Epic; }
        if s >= 120 { r = Rarity::Legendary; }
        if s >= 180 { r = Rarity::Mythic; }
        r
    }

    fn make_item(threat: u32, rarity: Rarity, turn: u8) -> Loot {
        let mut p = 10u32;
        let mut d = 10u32;
        let mut u = 1u32;
        if matches!(rarity, Rarity::Rare) { p = p.saturating_add(20); d = d.saturating_add(15); u = u.saturating_add(1); }
        if matches!(rarity, Rarity::Epic) { p = p.saturating_add(40); d = d.saturating_add(35); u = u.saturating_add(2); }
        if matches!(rarity, Rarity::Legendary) { p = p.saturating_add(70); d = d.saturating_add(60); u = u.saturating_add(3); }
        if matches!(rarity, Rarity::Mythic) { p = p.saturating_add(120); d = d.saturating_add(110); u = u.saturating_add(4); }
        let spice = ((threat.rotate_left((turn % 11) as u32)) % 23) as u32;
        Loot { rarity, power: p.saturating_add(spice), defense: d.saturating_add(spice / 2), utility: u.saturating_add(spice % 3) }
    }

    fn push_drops(log: &mut Account<RaidLog>, new_items: &Vec<Loot>) {
        let mut i = 0usize;
        while i < new_items.len() {
            log.recent.push(new_items[i].clone());
            i = i.saturating_add(1);
        }
        while log.recent.len() > 16 { log.recent.remove(0); }
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

#[account] pub struct ResourceHub { pub owner: Pubkey, pub created_at: i64, pub tier: TerritoryTier, pub nodes: Vec<ResourceNode> }
impl ResourceHub { pub const MAX_SPACE: usize = 32 + 8 + 1 + (4 + 5 * ResourceNode::SIZE); }

#[account] pub struct RaidLog { pub recent: Vec<Loot> }
impl RaidLog { pub const MAX_SPACE: usize = 4 + 16 * Loot::SIZE; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ResourceNode { pub resource_type: ResourceType, pub current_amount: u32, pub generation_rate: u32, pub capacity: u32, pub last_harvest: i64, pub enhancement_level: u8 }
impl ResourceNode { pub const SIZE: usize = 1 + 4 + 4 + 4 + 8 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier { Outpost, City, Capital }
#[derive(AnchorSerialize, AnchorDeserialize, Clone)] pub enum ResourceType { Gold, Crystal, Wood, Stone, MagicEssence }
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RaidDifficulty { Normal, Hard, Mythic }
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Rarity { Common, Rare, Epic, Legendary, Mythic }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Loot { pub rarity: Rarity, pub power: u32, pub defense: u32, pub utility: u32 }
impl Loot { pub const SIZE: usize = 1 + 4 + 4 + 4; }

#[error_code]
pub enum ErrorCode { #[msg("Party too small")] PartyTooSmall, #[msg("Party too large")] PartyTooLarge, #[msg("Invalid hours")] InvalidHours }
