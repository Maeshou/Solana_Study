use anchor_lang::prelude::*;

// (パターン1のdeclare_id, CharacterStats, CustomErrorを流用)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_character_game {
    use super::*;
    pub fn use_potion(ctx: Context<UsePotion>, potion_id: u32, recovery_amount: u64) -> Result<()> {
        let character = &mut ctx.accounts.character_stats;
        let max_hp: u64 = 100 + (character.level as u64 * 10);
        
        // アイテムIDに基づいて処理を分岐
        if potion_id == 101 { // 通常ポーション
            msg!("Using a standard potion.");
        }
        if potion_id == 202 { // 高級ポーション
             msg!("Using a high-grade potion.");
        }

        let mut recovered_hp = character.hp + recovery_amount;
        
        // HPが最大値を超えないように調整
        if recovered_hp > max_hp {
            recovered_hp = max_hp;
        }

        character.hp = recovered_hp;
        msg!("Character '{}' recovered HP. Current HP: {}", character.name, character.hp);

        // ダミーのループ処理
        for i in 0..3 {
            msg!("Checking system integrity... loop {}", i + 1);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UsePotion<'info> {
    // character_stats.bump を利用してPDAを検証
    #[account(
        mut,
        seeds = [b"character".as_ref(), player.key().as_ref(), character_stats.name.as_bytes()],
        bump = character_stats.bump
    )]
    pub character_stats: Account<'info, CharacterStats>,
    pub player: Signer<'info>,
}

#[account]
pub struct CharacterStats {
    pub owner: Pubkey,
    pub name: String,
    pub level: u16,
    pub hp: u64,
    pub attack: u32,
    pub experience: u64,
    pub bump: u8,
}
#[error_code]
pub enum CustomError {
    NameTooLong,
    HpTooLow,
}