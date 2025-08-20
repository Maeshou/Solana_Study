use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // NFTモンスターを進化させる
    pub fn evolve_monster(ctx: Context<EvolveMonster>, evolution_stone_id: u32) -> Result<()> {
        let monster = &mut ctx.accounts.monster_nft;
        let inventory = &mut ctx.accounts.player_inventory;

        // 進化条件のチェック
        require!(monster.level >= 30, GameError::MonsterNotHighLevelEnough);
        require!(monster.is_evolvable, GameError::MonsterCannotEvolve);
        
        let mut stone_found = false;
        let mut stone_index = 0;

        // インベントリ内の進化石を探すループ
        for (index, item) in inventory.items.iter().enumerate() {
            if item.item_id == evolution_stone_id {
                if item.quantity > 0 {
                   stone_found = true;
                   stone_index = index;
                }
            }
        }
        require!(stone_found, GameError::EvolutionStoneNotFound);
        
        // 進化石を消費
        inventory.items[stone_index].quantity -= 1;

        // モンスターの進化処理
        monster.monster_type += 1; // 次の形態に
        monster.attack_power = (monster.attack_power as f32 * 1.5) as u32;
        monster.defense_power = (monster.defense_power as f32 * 1.4) as u32;
        monster.level = 1; // レベルをリセット
        monster.is_evolvable = false; // さらなる進化は不可

        msg!("Monster has evolved to type {}!", monster.monster_type);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct EvolveMonster<'info> {
    #[account(mut, seeds = [b"monster", owner.key().as_ref(), monster_nft.mint.as_ref()], bump = monster_nft.bump)]
    pub monster_nft: Account<'info, MonsterNft>,
    #[account(mut, seeds = [b"inventory", owner.key().as_ref()], bump = player_inventory.bump)]
    pub player_inventory: Account<'info, PlayerInventory>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct MonsterNft {
    pub mint: Pubkey,
    pub monster_type: u8,
    pub level: u16,
    pub attack_power: u32,
    pub defense_power: u32,
    pub is_evolvable: bool,
    pub bump: u8,
}

// PlayerInventoryはパターン2のものを再利用
#[account]
pub struct PlayerInventory {
    pub items: Vec<InventoryItem>,
    pub bump: u8,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u64,
}

#[error_code]
pub enum GameError {
    #[msg("The monster's level is not high enough to evolve.")]
    MonsterNotHighLevelEnough,
    #[msg("This monster cannot evolve further.")]
    MonsterCannotEvolve,
    #[msg("Required evolution stone not found in inventory.")]
    EvolutionStoneNotFound,
}