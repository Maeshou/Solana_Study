use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;
    // キャラクターをギルドに参加させる
    pub fn join_guild(ctx: Context<JoinGuild>) -> Result<()> {
        let character = &ctx.accounts.character_stats;
        let guild = &mut ctx.accounts.guild_data;
        let player = &ctx.accounts.player;

        // ギルドの参加条件をチェック
        require!(character.level >= guild.minimum_level_to_join, GameError::LevelTooLowForGuild);
        require!(guild.member_count < guild.max_members, GameError::GuildIsFull);
        
        // 既にメンバーでないかを確認するループ
        for member in guild.members.iter() {
            require!(*member != player.key(), GameError::AlreadyInGuild);
        }

        // 新しいメンバーとして追加
        guild.members.push(player.key());
        guild.member_count += 1;
        
        // ギルドの合計パワーにキャラクターの戦力を加算
        let character_power = character.attack_power + character.defense_power;
        guild.total_power += u64::from(character_power);

        msg!("Welcome to the guild! Player {} has joined.", player.key());
        msg!("Guild total power is now: {}", guild.total_power);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct JoinGuild<'info> {
    #[account(seeds = [b"character_stats", player.key().as_ref()], bump = character_stats.bump)]
    pub character_stats: Account<'info, CharacterStats>,
    #[account(mut, seeds = [b"guild", guild_data.name.as_bytes()], bump = guild_data.bump)]
    pub guild_data: Account<'info, GuildData>,
    #[account(mut)]
    pub player: Signer<'info>,
}

// CharacterStatsはパターン1のものを再利用
#[account]
pub struct CharacterStats {
    pub level: u16,
    pub experience: u64,
    pub attack_power: u32,
    pub defense_power: u32,
    pub special_abilities: Vec<u8>,
    pub bump: u8,
}

#[account]
pub struct GuildData {
    pub name: String,
    pub members: Vec<Pubkey>,
    pub member_count: u16,
    pub max_members: u16,
    pub minimum_level_to_join: u16,
    pub total_power: u64,
    pub bump: u8,
}

#[error_code]
pub enum GameError {
    #[msg("Your character level is too low to join this guild.")]
    LevelTooLowForGuild,
    #[msg("This guild is already full.")]
    GuildIsFull,
    #[msg("You are already a member of this guild.")]
    AlreadyInGuild,
}