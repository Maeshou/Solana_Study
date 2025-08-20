use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("NFTGameVuln5555555555555555555555555555555");

#[program]
pub mod quest_reward_revival_demo {
    use super::*;

    pub fn claim_quest_reward(ctx: Context<ClaimQuestReward>) -> Result<()> {
        // クエスト報酬を受け取って冒険者に送る
        Ok(())
    }

    pub fn regenerate_quest_same_tx(
        ctx: Context<RegenerateQuestSameTx>,
        storage_capacity: u64,
        reward_multiplier: u32,
    ) -> Result<()> {
        let quest_account = ctx.accounts.quest_reward_addr.to_account_info();
        let quest_giver = ctx.accounts.quest_giver.to_account_info();

        let base_funding = 1_500_000;
        let multiplied_amount = base_funding + (reward_multiplier as u64 * 200_000);
        
        let regenerate_funds = system_instruction::transfer(
            &quest_giver.key(),
            &quest_account.key(),
            multiplied_amount
        );
        anchor_lang::solana_program::program::invoke(
            &regenerate_funds,
            &[quest_giver.clone(), quest_account.clone()],
        )?;

        let expand_quest_data = system_instruction::allocate(&quest_account.key(), storage_capacity);
        anchor_lang::solana_program::program::invoke(
            &expand_quest_data,
            &[quest_account.clone()]
        )?;

        let assign_quest_control = system_instruction::assign(&quest_account.key(), &crate::id());
        anchor_lang::solana_program::program::invoke(
            &assign_quest_control,
            &[quest_account.clone()]
        )?;

        let mut quest_data = quest_account.try_borrow_mut_data()?;
        let multiplier_bytes = reward_multiplier.to_be_bytes();
        let mut data_offset = 0;
        for multiplier_byte in multiplier_bytes.iter() {
            quest_data[data_offset] = *multiplier_byte;
            data_offset += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimQuestReward<'info> {
    #[account(mut, close = adventurer_wallet)]
    pub quest_reward: Account<'info, QuestRewardData>,
    #[account(mut)]
    pub adventurer_wallet: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RegenerateQuestSameTx<'info> {
    #[account(mut)]
    pub quest_reward_addr: UncheckedAccount<'info>,
    #[account(mut)]
    pub quest_giver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct QuestRewardData {
    pub experience_gained: u32,
    pub gold_reward: u64,
}