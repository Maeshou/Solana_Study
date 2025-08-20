// ──────────────────────────────────────────────────────────────────────────────
// 2) player_profiles: プレイヤープロファイルと装備の管理
// ──────────────────────────────────────────────────────────────────────────────
use anchor_lang::prelude::*;

declare_id!("PlAyErPrOfIlE1111111111111111111111111");

#[program]
pub mod player_profiles {
    use super::*;

    pub fn init_profile(ctx: Context<InitProfile>, player_name: String, skill: u64) -> Result<()> {
        let p = &mut ctx.accounts.profile;
        p.player = ctx.accounts.player.key();
        p.player_name = player_name;
        p.skill_level = skill;
        p.wins = 0;
        p.losses = 0;
        p.current_win_streak = 0;
        p.championships_won = 0;
        p.tournament_earnings = 0;
        p.elimination_round = 0;
        p.equipped_items = vec![];
        Ok(())
    }

    pub fn add_equipment(
        ctx: Context<MutProfile>,
        item_id: u32,
        kind: u8,        // 0:Weapon, 1:Armor, 2:Accessory（簡略）
        power: u64,
        enhance: u32,
    ) -> Result<()> {
        let pf = &mut ctx.accounts.profile;
        let eq = Equipment {
            item_id,
            equipment_type: EquipmentType::from_index(kind),
            power_level: power,
            enhancement_level: enhance,
        };
        pf.equipped_items.push(eq);
        Ok(())
    }

    pub fn record_result(ctx: Context<MutProfile>, win: bool, reward: u64, round: u32) -> Result<()> {
        let pf = &mut ctx.accounts.profile;
        if win {
            pf.wins = pf.wins.saturating_add(1);
            pf.current_win_streak = pf.current_win_streak.saturating_add(1);
            pf.tournament_earnings = pf.tournament_earnings.saturating_add(reward);
        } else {
            pf.losses = pf.losses.saturating_add(1);
            pf.current_win_streak = 0;
            pf.elimination_round = round;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + PlayerProfile::MAX_SIZE
    )]
    pub profile: Account<'info, PlayerProfile>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MutProfile<'info> {
    #[account(mut, has_one = player)]
    pub profile: Account<'info, PlayerProfile>,
    pub player: Signer<'info>,
}

#[account]
pub struct PlayerProfile {
    pub player: Pubkey,
    pub player_name: String,
    pub skill_level: u64,
    pub wins: u32,
    pub losses: u32,
    pub current_win_streak: u32,
    pub championships_won: u32,
    pub tournament_earnings: u64,
    pub elimination_round: u32,
    pub equipped_items: Vec<Equipment>,
}

impl PlayerProfile {
    pub const MAX_ITEMS: usize = 16;
    pub const MAX_NAME: usize = 48;
    pub const MAX_SIZE: usize =
        32 + 4 + Self::MAX_NAME + 8 + 4 + 4 + 4 + 4 + 8 + 4
        + 4 + Self::MAX_ITEMS * Equipment::MAX_SIZE;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Equipment {
    pub item_id: u32,
    pub equipment_type: EquipmentType,
    pub power_level: u64,
    pub enhancement_level: u32,
}
impl Equipment {
    pub const MAX_SIZE: usize = 4 + 1 + 8 + 4; // 簡略
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum EquipmentType { Weapon, Armor, Accessory }
impl EquipmentType {
    pub fn from_index(i: u8) -> Self {
        if i == 0 { return EquipmentType::Weapon; }
        if i == 1 { return EquipmentType::Armor; }
        EquipmentType::Accessory
    }
}
