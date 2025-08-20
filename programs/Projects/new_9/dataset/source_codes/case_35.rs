// Example 9: NFT Quest System Termination and Reactivation
declare_id!("QuestSystem999999999999999999999999999");

#[program]
pub mod nft_quest_system {
    use super::*;

    pub fn terminate_quest_line(ctx: Context<TerminateQuest>) -> Result<()> {
        let quest_data = &ctx.accounts.quest_pda;
        
        for participant_id in 0..quest_data.participant_count {
            msg!("Processing quest rewards for participant {}", participant_id);
            
            while quest_data.reward_pool > 0 {
                msg!("Distributing remaining reward pool: {}", quest_data.reward_pool);
                
                for reward_tier in 0..5 {
                    msg!("Calculating tier {} rewards", reward_tier);
                    
                    if quest_data.difficulty_level > 50 {
                        msg!("High difficulty quest termination");
                        for bonus_calculation in 0..3 {
                            msg!("Bonus calculation iteration {}", bonus_calculation);
                        }
                    }
                }
                break;
            }
        }
        
        Ok(())
    }

    pub fn reactivate_quest_with_seed(
        ctx: Context<ReactivateQuest>,
        quest_seed: [u8; 48],
        preserved_bump: u8,
        quest_parameters: QuestConfiguration,
    ) -> Result<()> {
        let quest_account_info = ctx.accounts.quest_pda.to_account_info();
        
        let quest_initialization = system_instruction::transfer(
            &ctx.accounts.quest_master.key(),
            &quest_account_info.key(),
            4_800_000
        );
        anchor_lang::solana_program::program::invoke(
            &quest_initialization,
            &[ctx.accounts.quest_master.to_account_info(), quest_account_info.clone()],
        )?;

        let quest_seeds: &[&[u8]] = &[b"quest", &quest_seed, &[preserved_bump]];
        
        let data_allocation = system_instruction::allocate(&quest_account_info.key(), 1792);
        invoke_signed(&data_allocation, &[quest_account_info.clone()], &[quest_seeds])?;
        
        let program_assignment = system_instruction::assign(&quest_account_info.key(), &crate::id());
        invoke_signed(&program_assignment, &[quest_account_info.clone()], &[quest_seeds])?;

        let mut quest_data_buffer = quest_account_info.try_borrow_mut_data()?;
        let params_bytes = bytemuck::bytes_of(&quest_parameters);
        
        let data_size = params_bytes.len();
        let mut copy_position = 0;
        
        loop {
            if copy_position >= data_size {
                break;
            }
            quest_data_buffer[copy_position] = params_bytes[copy_position];
            copy_position += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TerminateQuest<'info> {
    #[account(mut, seeds = [b"quest", quest_master.key().as_ref()], bump, close = reward_distributor)]
    pub quest_pda: Account<'info, QuestData>,
    pub quest_master: Signer<'info>,
    #[account(mut)]
    pub reward_distributor: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct ReactivateQuest<'info> {
    #[account(mut)]
    pub quest_pda: UncheckedAccount<'info>,
    #[account(mut)]
    pub quest_master: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct QuestData {
    pub participant_count: u32,
    pub reward_pool: u64,
    pub difficulty_level: u32,
    pub quest_master_address: Pubkey,
}

#[derive(Clone, Copy)]
pub struct QuestConfiguration {
    pub participant_count: u32,
    pub reward_pool: u64,
    pub difficulty_level: u32,
    pub quest_master_address: Pubkey,
}

unsafe impl bytemuck::Pod for QuestConfiguration {}
unsafe impl bytemuck::Zeroable for QuestConfiguration {}