// ========================================
// 1. 脆弱なNFTバトルアリーナ - Vulnerable NFT Battle Arena
// ========================================

use anchor_lang::prelude::*;

declare_id!("V1uLnErAbLeCoD3F0r3xAmP1e5tUdY7BaTt1eAr3nA0x");

#[program]
pub mod vulnerable_battle_arena {
    use super::*;
    use FighterType::*;

    pub fn init_arena(ctx: Context<InitArena>) -> Result<()> {
        let arena = &mut ctx.accounts.arena;
        arena.owner = ctx.accounts.owner.key();
        arena.total_battles = 0;
        arena.prize_pool = 0;
        Ok(())
    }

    pub fn register_fighter(ctx: Context<RegisterFighter>, fighter_type: FighterType) -> Result<()> {
        let fighter = &mut ctx.accounts.fighter;
        fighter.arena = ctx.accounts.arena.key();
        fighter.owner = ctx.accounts.owner.key();
        fighter.fighter_type = fighter_type;
        fighter.power = 100;
        fighter.wins = 0;
        Ok(())
    }

    // 脆弱性: AccountInfoを使用し、型検証が不十分
    pub fn battle_exploit(ctx: Context<VulnerableBattle>) -> Result<()> {
        let attacker_info = &ctx.accounts.attacker;
        let defender_info = &ctx.accounts.defender;
        let arena = &mut ctx.accounts.arena;

        // 脆弱性: AccountInfoから直接データを読み込み、型安全性なし
        let attacker_data = attacker_info.try_borrow_data()?;
        let defender_data = defender_info.try_borrow_data()?;

        // 脆弱性: discriminator検証なしでデータを解釈
        if attacker_data.len() >= 41 && defender_data.len() >= 41 {
            let attacker_power = u32::from_le_bytes([
                attacker_data[33], attacker_data[34], 
                attacker_data[35], attacker_data[36]
            ]);
            let defender_power = u32::from_le_bytes([
                defender_data[33], defender_data[34], 
                defender_data[35], defender_data[36]
            ]);

            // バトル処理ループ
            for round in 0..3 {
                if attacker_power > defender_power {
                    arena.prize_pool = arena.prize_pool.checked_add((round + 1) as u64 * 100).unwrap_or(u64::MAX);
                    arena.total_battles = arena.total_battles.checked_add(1).unwrap_or(u64::MAX);
                    msg!("Attacker wins round {}", round);
                } else {
                    arena.total_battles = arena.total_battles.checked_add(2).unwrap_or(u64::MAX);
                    arena.prize_pool = arena.prize_pool.checked_add(50).unwrap_or(u64::MAX);
                    msg!("Defender wins round {}", round);
                }
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 8 + 8)]
    pub arena: Account<'info, Arena>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterFighter<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    #[account(init, payer = owner, space = 8 + 32 + 32 + 1 + 4 + 4)]
    pub fighter: Account<'info, Fighter>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 脆弱性: AccountInfoを使用、型安全性なし
#[derive(Accounts)]
pub struct VulnerableBattle<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    /// CHECK: 脆弱性 - 型検証なしのAccountInfo使用
    pub attacker: AccountInfo<'info>,
    /// CHECK: 脆弱性 - 型検証なしのAccountInfo使用  
    pub defender: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Arena {
    pub owner: Pubkey,
    pub total_battles: u64,
    pub prize_pool: u64,
}

#[account]
pub struct Fighter {
    pub arena: Pubkey,
    pub owner: Pubkey,
    pub fighter_type: FighterType,
    pub power: u32,
    pub wins: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum FighterType {
    Warrior,
    Mage,
    Archer,
    Assassin,
}