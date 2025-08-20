use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_game {
    use super::*;

    // NFTペットを訓練する
    pub fn train_pet(ctx: Context<TrainPet>, training_sessions: u8) -> Result<()> {
        let pet = &mut ctx.accounts.pet_nft;
        
        // 訓練コストを計算 (1セッションあたり100トークン)
        let total_cost = 100 * u64::from(training_sessions);

        // 訓練コストの支払い処理
        let cpi_accounts = Transfer {
            from: ctx.accounts.player_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, total_cost)?;

        // 訓練セッションの回数だけループ
        for i in 0..training_sessions {
            // 訓練によるステータス上昇
            pet.loyalty += 1;
            
            // 訓練の成功判定 (擬似乱数)
            let clock = Clock::get()?;
            let pseudo_random = (clock.unix_timestamp as u64).wrapping_add(i as u64) % 10;

            if pseudo_random < 4 { // 40%の確率で素早さアップ
                pet.agility += 2;
                msg!("Session {}: Agility increased!", i + 1);
            }
            
            if pseudo_random > 6 { // 30%の確率で賢さアップ
                pet.intelligence += 2;
                msg!("Session {}: Intelligence increased!", i + 1);
            }

            // 忠誠度が一定値を超えると特殊スキルを覚える
            if pet.loyalty == 50 {
                pet.skills.push(101); // スキルID 101
                msg!("Pet learned a new skill due to high loyalty!");
            }
        }
        
        msg!("Pet training complete. New Agility: {}, New Intelligence: {}", pet.agility, pet.intelligence);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TrainPet<'info> {
    #[account(mut, seeds = [b"pet", owner.key().as_ref(), pet_nft.mint.as_ref()], bump = pet_nft.bump)]
    pub pet_nft: Account<'info, PetNft>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>, // 運営のトークンアカウント
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct PetNft {
    pub mint: Pubkey,
    pub loyalty: u16,
    pub agility: u32,
    pub intelligence: u32,
    pub skills: Vec<u16>,
    pub bump: u8,
}