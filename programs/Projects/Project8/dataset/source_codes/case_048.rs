use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_character_game {
    use super::*;

    // 新規キャラクターを作成する
    pub fn create_character(ctx: Context<CreateCharacter>, name: String, initial_hp: u64, initial_attack: u32) -> Result<()> {
        let character = &mut ctx.accounts.character_stats;
        
        require!(name.len() < 20, CustomError::NameTooLong);
        require!(initial_hp > 50, CustomError::HpTooLow);
        
        character.owner = *ctx.accounts.player.key;
        character.name = name;
        character.level = 1;
        character.hp = initial_hp;
        character.attack = initial_attack;
        character.experience = 0;
        // アカウント作成時に導出された正準なbumpを保存する
        character.bump = *ctx.bumps.get("character_stats").unwrap();

        msg!("New character '{}' created!", character.name);
        msg!("HP: {}, Attack: {}", character.hp, character.attack);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateCharacter<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 4 + 20 + 2 + 8 + 4 + 8 + 1, // Discriminator + owner + name_len + name + level + hp + attack + exp + bump
        seeds = [b"character".as_ref(), player.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub character_stats: Account<'info, CharacterStats>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CharacterStats {
    pub owner: Pubkey,
    pub name: String,
    pub level: u16,
    pub hp: u64,
    pub attack: u32,
    pub experience: u64,
    pub bump: u8, // Bump Seedを保存するフィールド
}

#[error_code]
pub enum CustomError {
    #[msg("Character name is too long.")]
    NameTooLong,
    #[msg("Initial HP must be greater than 50.")]
    HpTooLow,
}