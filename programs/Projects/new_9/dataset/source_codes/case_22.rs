use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("NFTGameVuln1111111111111111111111111111111");

#[program]
pub mod monster_hatchery {
    use super::*;

    // 新しいモンスターの卵を生成する（脆弱な再生成処理）
    pub fn create_new_egg(ctx: Context<CreateNewEgg>, power_level: u32, hatch_time: u64) -> ProgramResult {
        let egg_account = ctx.accounts.monster_egg.to_account_info();
        let creator = ctx.accounts.creator.to_account_info();

        // 卵のパワーレベルに応じて生成コストを計算
        let base_cost = 500_000;
        let power_bonus = (power_level as u64) * 10_000;
        let creation_fee = base_cost + power_bonus;

        // アカウントへのSOL転送
        let transfer_instruction = system_instruction::transfer(&creator.key(), &egg_account.key(), creation_fee);
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[creator.clone(), egg_account.clone()],
        )?;

        // アカウントのスペース確保と所有権の割り当て
        let egg_space = 8 + 4 + 8; // Discriminator + power_level + hatch_time
        let allocate_instruction = system_instruction::allocate(&egg_account.key(), egg_space);
        anchor_lang::solana_program::program::invoke(
            &allocate_instruction,
            &[egg_account.clone()]
        )?;
        let assign_instruction = system_instruction::assign(&egg_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &assign_instruction,
            &[egg_account.clone()]
        )?;
        
        // 手動でデータを書き込む
        let mut account_data = egg_account.try_borrow_mut_data()?;
        // Discriminatorをスキップして書き込み
        let mut data_slice = &mut account_data[8..];
        data_slice[..4].copy_from_slice(&power_level.to_le_bytes());
        data_slice[4..12].copy_from_slice(&hatch_time.to_le_bytes());

        Ok(())
    }

    // 卵を孵化させてアカウントをクローズする
    pub fn hatch_monster_egg(ctx: Context<HatchMonsterEgg>) -> ProgramResult {
        // 孵化のロジック（この場合はアカウントを閉じるだけ）
        Ok(())
    }
}

#[derive(Accounts)]
pub struct HatchMonsterEgg<'info> {
    #[account(mut, close = egg_layer)]
    pub monster_egg: Account<'info, MonsterEgg>,
    #[account(mut)]
    pub egg_layer: Signer<'info>,
}

#[account]
pub struct MonsterEgg {
    pub power_level: u32,
    pub hatch_time: u64,
}

#[derive(Accounts)]
pub struct CreateNewEgg<'info> {
    #[account(mut)]
    pub monster_egg: UncheckedAccount<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}