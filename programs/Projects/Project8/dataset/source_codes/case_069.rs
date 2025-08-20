use anchor_lang::prelude::*;

declare_id!("G1LdBaSeReg11111111111111111111111111111");

#[program]
pub mod guild_base_registry {
    use super::*;

    pub fn register_guild(
        ctx: Context<RegisterGuild>,
        guild_name: String,
        difficulty: DifficultyLevel,
        initial_treasury: u64,
    ) -> Result<()> {
        let base = &mut ctx.accounts.guild_base;
        let now = Clock::get()?.unix_timestamp;

        require!(guild_name.len() > 0, ErrorCode::EmptyName);
        require!(guild_name.len() <= 32, ErrorCode::NameTooLong);
        require!(initial_treasury > 0, ErrorCode::TreasuryTooLow);

        base.guild_master = ctx.accounts.guild_master.key();
        base.name = guild_name;
        base.established_at = now;
        base.members = 1;
        base.treasury = initial_treasury;
        base.difficulty = difficulty.clone();

        // 段階的上書き（固定順で決め打ちしない）：初期値→難易度で徐々に上書き
        let mut exp = 100u32;
        let mut res = 100u32;
        let mut def = 100u32;
        let mut atk = 100u32;

        if matches_easy(&difficulty) {
            exp = exp.saturating_add(10);
            res = res.saturating_add(5);
        }
        if matches_normal(&difficulty) {
            exp = exp.saturating_add(5);
            def = def.saturating_add(5);
        }
        if matches_hard(&difficulty) {
            atk = atk.saturating_add(10);
            def = def.saturating_add(10);
        }
        if matches_legend(&difficulty) {
            exp = exp.saturating_add(20);
            res = res.saturating_add(15);
            atk = atk.saturating_add(20);
        }

        // 名前長や時刻からseed的なブレンドで微調整（過度な定数を避け、上書きで整える）
        let name_len = base.name.len() as u32;
        let tick_bias = (now as u64).rotate_left(7) as u32 % 9;
        base.bonus.experience_multiplier = exp.saturating_add(name_len % 7);
        base.bonus.resource_bonus = res.saturating_add(tick_bias % 5);
        base.bonus.defense_bonus = def.saturating_add((name_len ^ tick_bias) % 6);
        base.bonus.attack_bonus = atk.saturating_add((tick_bias ^ 3) % 7);

        Ok(())
    }

    fn matches_easy(d: &DifficultyLevel) -> bool { matches!(d, DifficultyLevel::Easy) }
    fn matches_normal(d: &DifficultyLevel) -> bool { matches!(d, DifficultyLevel::Normal) }
    fn matches_hard(d: &DifficultyLevel) -> bool { matches!(d, DifficultyLevel::Hard) }
    fn matches_legend(d: &DifficultyLevel) -> bool { matches!(d, DifficultyLevel::Legend) }
}

#[derive(Accounts)]
pub struct RegisterGuild<'info> {
    #[account(
        init,
        payer = guild_master,
        space = 8 + GuildBase::MAX_SPACE,
        seeds = [b"guild-base", guild_master.key().as_ref()],
        bump
    )]
    pub guild_base: Account<'info, GuildBase>,
    #[account(mut)]
    pub guild_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GuildBase {
    pub guild_master: Pubkey,
    pub name: String,
    pub established_at: i64,
    pub members: u32,
    pub treasury: u64,
    pub difficulty: DifficultyLevel,
    pub bonus: GuildBonus,
}
impl GuildBase {
    pub const MAX_SPACE: usize = 32 + 4 + 32 + 8 + 4 + 8 + 1 + GuildBonus::SIZE;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GuildBonus {
    pub experience_multiplier: u32,
    pub resource_bonus: u32,
    pub defense_bonus: u32,
    pub attack_bonus: u32,
}
impl GuildBonus { pub const SIZE: usize = 4 * 4; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum DifficultyLevel { Easy, Normal, Hard, Legend }

#[error_code]
pub enum ErrorCode {
    #[msg("Guild name must not be empty")]
    EmptyName,
    #[msg("Guild name is too long")]
    NameTooLong,
    #[msg("Initial treasury must be positive")]
    TreasuryTooLow,
}
