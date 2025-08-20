// 1) dungeon_registry_core: インスタンス基本生成（難易度パラメータとパーティ検証）
use anchor_lang::prelude::*;

declare_id!("DungCore33333333333333333333333333333333");

#[program]
pub mod dungeon_registry_core {
    use super::*;

    pub fn create_instance(
        ctx: Context<CreateInstance>,
        difficulty: DungeonDifficulty,
        party: Vec<PartyMember>,
        entrance_fee: u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // パーティサイズ検証（&& を使わず個別に）
        let size = party.len();
        require!(size >= 1, DungeonError::PartyTooSmall);
        require!(size <= 6, DungeonError::PartyTooLarge);

        // 初期値（Beginner 相当）を置いてから段階的に上書き
        let mut rec_min: u32 = 1;
        let mut rec_max: u32 = 10;
        let mut time_limit: u32 = 1800;
        let mut base_xp: u32 = 500;
        let mut base_mobs: u32 = 15;

        // Intermediate
        if is_intermediate(difficulty) {
            rec_min = 11; rec_max = 25; time_limit = 2700; base_xp = 1200; base_mobs = 25;
        }
        // Advanced
        if is_advanced(difficulty) {
            rec_min = 26; rec_max = 40; time_limit = 3600; base_xp = 2500; base_mobs = 35;
        }
        // Expert
        if is_expert(difficulty) {
            rec_min = 41; rec_max = 60; time_limit = 4500; base_xp = 4000; base_mobs = 50;
        }
        // Legendary
        if is_legendary(difficulty) {
            rec_min = 61; rec_max = 100; time_limit = 5400; base_xp = 7500; base_mobs = 75;
        }

        let header = &mut ctx.accounts.header;
        header.instance_creator = ctx.accounts.party_leader.key();
        header.creation_time = now;
        header.difficulty_level = difficulty;
        header.party_members = party;
        header.entrance_fee = entrance_fee;
        header.recommended_level_range = (rec_min, rec_max);
        header.completion_time_limit = time_limit;
        header.base_experience_reward = base_xp;
        header.base_monster_count = base_mobs;
        header.completion_status = CompletionStatus::InProgress;

        Ok(())
    }

    fn is_intermediate(d: DungeonDifficulty) -> bool { if let DungeonDifficulty::Intermediate = d { return true } false }
    fn is_advanced(d: DungeonDifficulty) -> bool { if let DungeonDifficulty::Advanced = d { return true } false }
    fn is_expert(d: DungeonDifficulty) -> bool { if let DungeonDifficulty::Expert = d { return true } false }
    fn is_legendary(d: DungeonDifficulty) -> bool { if let DungeonDifficulty::Legendary = d { return true } false }
}

#[derive(Accounts)]
pub struct CreateInstance<'info> {
    #[account(
        init,
        payer = party_leader,
        space = 8 + DungeonHeader::LEN,
        seeds = [b"dungeon_header", party_leader.key().as_ref()],
        bump
    )]
    pub header: Account<'info, DungeonHeader>,
    #[account(mut)]
    pub party_leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DungeonHeader {
    pub instance_creator: Pubkey,
    pub creation_time: i64,
    pub difficulty_level: DungeonDifficulty,
    pub party_members: Vec<PartyMember>,
    pub entrance_fee: u64,
    pub recommended_level_range: (u32, u32),
    pub completion_time_limit: u32,
    pub base_experience_reward: u32,
    pub base_monster_count: u32,
    pub completion_status: CompletionStatus,
}
impl DungeonHeader {
    pub const LEN: usize = 32 + 8 + 1 + (4 + 6 * PartyMember::LEN) + 8 + (4 + 4) + 4 + 4 + 4 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum DungeonDifficulty { Beginner, Intermediate, Advanced, Expert, Legendary }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PartyMember {
    pub player_pubkey: Pubkey,
    pub character_level: u32,
    pub character_class: CharacterClass,
}
impl PartyMember { pub const LEN: usize = 32 + 4 + 1; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum CharacterClass { Warrior, Mage, Rogue, Cleric, Archer, Paladin }

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum CompletionStatus { InProgress, Cleared, Failed }

#[error_code]
pub enum DungeonError {
    #[msg("Party size is too small")] PartyTooSmall,
    #[msg("Party size is too large")] PartyTooLarge,
}
