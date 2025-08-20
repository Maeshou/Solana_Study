use anchor_lang::prelude::*;

// (パターン1のdeclare_id, CharacterStats, CustomErrorを流用)
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_character_game {
    use super::*;
    pub fn complete_quest(ctx: Context<CompleteQuest>, earned_exp: u64) -> Result<()> {
        let character = &mut ctx.accounts.character_stats;
        
        character.experience += earned_exp;
        msg!("Quest completed! '{}' earned {} EXP.", character.name, earned_exp);
        msg!("Total EXP: {}", character.experience);

        let mut leveled_up = false;
        // レベルアップに必要な経験値 (例: レベル * 100)
        let mut required_exp = character.level as u64 * 100;
        
        // 複数回レベルアップする可能性を考慮してwhileループを使用
        while character.experience >= required_exp {
            character.level += 1;
            character.experience -= required_exp;
            
            // レベルアップボーナス
            character.hp += 20;
            character.attack += 5;
            
            leveled_up = true;
            msg!("Level up! '{}' is now level {}", character.name, character.level);
            
            // 次のレベルアップに必要な経験値を更新
            required_exp = character.level as u64 * 100;
        }
        
        if leveled_up {
            msg!("New Stats -> HP: {}, Attack: {}", character.hp, character.attack);
        }

        Ok(())
    }
}


#[derive(Accounts)]
pub struct CompleteQuest<'info> {
    // character_stats.bump を利用してPDAを検証
    #[account(
        mut,
        seeds = [b"character".as_ref(), owner.key().as_ref(), character_stats.name.as_bytes()],
        bump = character_stats.bump,
        has_one = owner
    )]
    pub character_stats: Account<'info, CharacterStats>,
    pub owner: Signer<'info>,
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