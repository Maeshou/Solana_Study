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

        // ティアは段階的上書き（固定のif-else連鎖を避ける）
        atlas.tier = TerritoryTier::Outpost;
        if area >= 2_000 { atlas.tier = TerritoryTier::City; }
        if area >= 5_000 { atlas.tier = TerritoryTier::Capital; }

        // seed由来のばらつき：座標から擬似バイアスを導出してdailyを上書き
        let mut daily = base_daily.max(10);
        let bias1 = (coords.north_boundary as i64 - coords.south_boundary as i64).unsigned_abs() as u32;
        let bias2 = (coords.east_boundary as i64 - coords.west_boundary as i64).unsigned_abs() as u32;
        daily = daily.saturating_add((bias1 % 13) + (bias2 % 7));

        // ティア別の加点も「加算」で上書き
        if matches_capital(&atlas.tier) { daily = daily.saturating_add(200); }
        if matches_city(&atlas.tier) { daily = daily.saturating_add(90); }
        if matches_outpost(&atlas.tier) { daily = daily.saturating_add(30); }

        atlas.daily_baseline = daily;

        // 順序固定回避：後工程が使えるshuffle_seedを保存（座標と時間から）
        let seed_mix = ((now as u64).rotate_left(11)) ^ ((area as u64).rotate_left(3));
        atlas.shuffle_seed = seed_mix;

        Ok(())
    }

    fn matches_capital(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::Capital) }
    fn matches_city(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::City) }
    fn matches_outpost(t: &TerritoryTier) -> bool { matches!(t, TerritoryTier::Outpost) }
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
pub struct TerritoryCoordinates {
    pub north_boundary: i32,
    pub south_boundary: i32,
    pub east_boundary: i32,
    pub west_boundary: i32,
}
impl TerritoryCoordinates {
    pub const SIZE: usize = 4 * 4;
    pub fn area(&self) -> u32 {
        let w = (self.east_boundary - self.west_boundary).abs() as u32;
        let h = (self.north_boundary - self.south_boundary).abs() as u32;
        w.saturating_mul(h)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TerritoryTier { Outpost, City, Capital }

#[error_code]
pub enum ErrorCode {
    #[msg("Territory area too small")] TooSmall,
    #[msg("Territory area too large")] TooLarge,
}
