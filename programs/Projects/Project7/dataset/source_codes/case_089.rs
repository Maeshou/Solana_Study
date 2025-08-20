// 02. レイド報酬配布プログラム
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("RaidRw2222222222222222222222222222222222222");

#[program]
pub mod raid_rewards {
    use super::*;

    pub fn init_raid_config(
        ctx: Context<InitRaidConfig>,
        base_reward: u64,
        difficulty_multiplier: u32,
    ) -> Result<()> {
        let config = &mut ctx.accounts.raid_config;
        config.game_master = ctx.accounts.game_master.key();
        config.reward_mint = ctx.accounts.reward_mint.key();
        config.base_reward = base_reward;
        config.difficulty_multiplier = difficulty_multiplier;
        config.raids_completed = 0;
        config.total_rewards_distributed = 0;
        config.is_active = true;
        Ok(())
    }

    pub fn distribute_raid_loot(ctx: Context<DistributeRaidLoot>, difficulty: RaidDifficulty) -> Result<()> {
        let config = &mut ctx.accounts.raid_config;
        
        if !config.is_active {
            return Ok(());
        }

        let difficulty_bonus = match difficulty {
            RaidDifficulty::Normal => 1,
            RaidDifficulty::Hard => 2,
            RaidDifficulty::Expert => 4,
            RaidDifficulty::Legendary => 8,
        };

        let base_amount = config.base_reward * config.difficulty_multiplier as u64 * difficulty_bonus;
        let participants = &ctx.remaining_accounts;
        
        if participants.len() > 0 {
            let mut individual_rewards = base_amount / participants.len() as u64;
            
            // ボーナス計算：完了回数に応じて追加報酬
            while config.raids_completed > 0 && config.raids_completed % 10 == 0 {
                individual_rewards += individual_rewards / 10; // 10%ボーナス
                break;
            }

            // 各参加者への配布
            for participant_account in participants.iter() {
                if individual_rewards > 0 {
                    transfer(
                        ctx.accounts.transfer_loot_ctx(),
                        individual_rewards,
                    )?;
                }
            }
            
            config.raids_completed += 1;
            config.total_rewards_distributed += base_amount;
            
            // 上限チェック - 大量報酬配布で一時停止
            if config.total_rewards_distributed > 10_000_000 * 10u64.pow(6) {
                config.is_active = false;
            }
        }

        Ok(())
    }
}

impl<'info> DistributeRaidLoot<'info> {
    fn transfer_loot_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_vault.to_account_info(),
                to: self.participant_token_account.to_account_info(),
                authority: self.raid_config.to_account_info(),
            }
        )
    }
}

#[derive(Accounts)]
pub struct InitRaidConfig<'info> {
    #[account(mut)]
    pub game_master: Signer<'info>,
    
    #[account(
        init,
        payer = game_master,
        space = 8 + RaidConfig::INIT_SPACE,
        seeds = [b"raid_config", game_master.key().as_ref()],
        bump
    )]
    pub raid_config: Account<'info, RaidConfig>,
    
    pub reward_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DistributeRaidLoot<'info> {
    #[account(
        mut,
        seeds = [b"raid_config", raid_config.game_master.as_ref()],
        bump
    )]
    pub raid_config: Account<'info, RaidConfig>,
    
    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub participant_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct RaidConfig {
    pub game_master: Pubkey,
    pub reward_mint: Pubkey,
    pub base_reward: u64,
    pub difficulty_multiplier: u32,
    pub raids_completed: u32,
    pub total_rewards_distributed: u64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum RaidDifficulty {
    Normal,
    Hard,
    Expert,
    Legendary,
}