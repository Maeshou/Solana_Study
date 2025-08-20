use anchor_lang::prelude::*;

declare_id!("TerR1t0RyAtLaS1111111111111111111111111");

#[program]
pub mod territory_atlas {
    use super::*;

    pub fn define_territory(
        ctx: Context<DefineTerritory>,
        coords: TerritoryCoordinates,
        base_daily: u32,
    ) -> Result<()> {
        let atlas = &mut ctx.accounts.territory;
        let now = Clock::get()?.unix_timestamp;

        let area = coords.area();
        require!(area >= 100, ErrorCode::TooSmall);
        require!(area <= 10_000, ErrorCode::TooLarge);

        atlas.owner = ctx.accounts.guild_master.key();
        atlas.coords = coords.clone();
        atlas.created_at = now;
        atlas.size = area;

        atlas.tier = tier_from_area(area);
        atlas.daily_baseline = blended_daily(coords.clone(), base_daily, &atlas.tier);
        atlas.shuffle_seed = mix_seed(now, area);

        Ok(())
    }

    fn tier_from_area(area: u32) -> TerritoryTier {
        let mut t = TerritoryTier::Outpost;
        if area >= 2_000 { t = TerritoryTier::City; }
        if area >= 5_000 { t = TerritoryTier::Capital; }
        t
    }
    fn blended_daily(coords: TerritoryCoordinates, base: u32, tier: &TerritoryTier) -> u32 {
        let mut daily = base.max(10);
        daily = daily.saturating_add(coord_bias(coords));
        daily = daily.saturating_add(tier_bias(tier));
        daily
    }
    fn coord_bias(c: TerritoryCoordinates) -> u32 {
        let bias1 = (c.north_boundary as i64 - c.south_boundary as i64).unsigned_abs() as u32;
        let bias2 = (c.east_boundary as i64 - c.west_boundary as i64).unsigned_abs() as u32;
        (bias1 % 13).saturating_add(bias2 % 7)
    }
    fn tier_bias(t: &TerritoryTier) -> u32 {
        let mut s = 0u32;
        if matches!(t, TerritoryTier::Capital) { s = s.saturating_add(200); }
        if matches!(t, TerritoryTier::City) { s = s.saturating_add(90); }
        if matches!(t, TerritoryTier::Outpost) { s = s.saturating_add(30); }
        s
    }
    fn mix_seed(now: i64, area: u32) -> u64 {
        ((now as u64).rotate_left(11)) ^ ((area as u64).rotate_left(3))
    }
}

#[derive(Accounts)]
pub struct DefineTerritory<'info> {
    #[account(
        init,
        payer = guild_master,
        space = 8 + Territory::MAX_SPACE,
        seeds = [b"territory", guild_master.key().as_ref()],
        bump
    )]
    pub territory: Account<'info, Territory>,
    #[account(mut)]
    pub guild_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Territory {
    pub owner: Pubkey,
    pub coords: TerritoryCoordinates,
    pub size: u32,
    pub tier: TerritoryTier,
    pub created_at: i64,
    pub daily_baseline: u32,
    pub shuffle_seed: u64,
}
impl Territory { pub const MAX_SPACE: usize = 32 + TerritoryCoordinates::SIZE + 4 + 1 + 8 + 4 + 8; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TerritoryCoordinates { pub north_boundary: i32, pub south_boundary: i32, pub east_boundary: i32, pub west_boundary: i32 }
impl TerritoryCoordinates {
    pub const SIZE: usize = 16;
    pub fn area(&self) -> u32 {
        let w = (self.east_boundary - self.west_boundary).abs() as u32;
        let h = (self.north_boundary - self.south_boundary).abs() as u32;
        w.saturating_mul(h)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier { Outpost, City, Capital }

#[error_code]
pub enum ErrorCode { #[msg("Territory too small")] TooSmall, #[msg("Territory too large")] TooLarge }
