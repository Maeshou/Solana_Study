// 02. レイド報酬の配布
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("6t9Ew5rU8vA7zB1qK3h0j9D2c4x6F8y1L5n7P4o3M2v1b0N9m8k7J6I5H4g3F2E1D");

#[program]
pub mod raid_rewards_distributor {
    use super::*;

    pub fn initialize_raid_event(ctx: Context<InitializeRaidEvent>, max_participants: u32, total_reward_amount: u64) -> Result<()> {
        let raid_event = &mut ctx.accounts.raid_event;
        raid_event.creator = ctx.accounts.creator.key();
        raid_event.reward_mint = ctx.accounts.reward_mint.key();
        raid_event.total_reward_amount = total_reward_amount;
        raid_event.max_participants = max_participants;
        raid_event.participants_count = 0;
        Ok(())
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>, ranks: Vec<u32>) -> Result<()> {
        let raid_event = &mut ctx.accounts.raid_event;
        let mut reward_per_participant = 0;
        if raid_event.participants_count > 0 {
            reward_per_participant = raid_event.total_reward_amount / raid_event.participants_count as u64;
        }

        let mut amount_to_transfer = 0;
        let mut loop_count = 0;
        let mut i = 0;

        while i < ranks.len() {
            loop_count += 1;
            if loop_count > raid_event.max_participants {
                break;
            }
            if ranks[i] <= 10 { // Top 10 get a bonus
                amount_to_transfer = reward_per_participant + 100;
            } else {
                amount_to_transfer = reward_per_participant;
            }

            if amount_to_transfer > 0 {
                let cpi_accounts = Transfer {
                    from: ctx.accounts.pool_account.to_account_info(),
                    to: ctx.accounts.participant_token_account.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                token::transfer(cpi_ctx, amount_to_transfer)?;
            }

            i += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(max_participants: u32, total_reward_amount: u64)]
pub struct InitializeRaidEvent<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 32 + 8 + 4 + 4)]
    pub raid_event: Account<'info, RaidEvent>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub reward_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut, has_one = creator)]
    pub raid_event: Account<'info, RaidEvent>,
    #[account(mut)]
    pub pool_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub participant_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RaidEvent {
    pub creator: Pubkey,
    pub reward_mint: Pubkey,
    pub total_reward_amount: u64,
    pub max_participants: u32,
    pub participants_count: u32,
}