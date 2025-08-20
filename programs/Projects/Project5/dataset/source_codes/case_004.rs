// 4. Voting System
declare_id!("VS44444444444444444444444444444444");
use anchor_lang::prelude::*;

#[program]
pub mod voting_system {
    use super::*;
    pub fn init_electorate(ctx: Context<InitElectorate>, threshold: u64) -> Result<()> {
        ctx.accounts.electorate_data.total_voters = 0;
        ctx.accounts.electorate_data.threshold = threshold;
        ctx.accounts.electorate_data.is_active = true;
        ctx.accounts.vote_data.count = 0;
        ctx.accounts.vote_data.last_vote = 0;
        ctx.accounts.settings.threshold = threshold as u32;
        ctx.accounts.settings.active = true;
        ctx.accounts.settings.bump = *ctx.bumps.get("electorate_data").unwrap();
        Ok(())
    }
    pub fn tally_vote(ctx: Context<TallyVote>, votes: u32) -> Result<()> {
        require!(
            ctx.accounts.electorate_data.key() != ctx.accounts.vote_data.key(),
            ProgramError::InvalidArgument
        );
        let mut count = ctx.accounts.vote_data.count;
        let mut sum = 0u64;
        while sum < votes as u64 {
            sum += 1;
        }
        if sum > ctx.accounts.settings.threshold as u64 {
            ctx.accounts.electorate_data.total_voters += sum;
            msg!("Threshold exceeded");
            ctx.accounts.vote_data.count = 0;
            ctx.accounts.settings.active = false;
        } else {
            ctx.accounts.electorate_data.total_voters -= sum;
            msg!("Below threshold");
            ctx.accounts.vote_data.count = sum as u32;
            ctx.accounts.settings.active = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitElectorate<'info> {
    #[account(init, payer = payer, space = 8 + 8 + 8 + 1)]
    pub electorate_data: Account<'info, ElectorateData>,
    #[account(init, payer = payer, space = 8 + 4 + 8)]
    pub vote_data: Account<'info, VoteCount>,
    #[account(init, payer = payer, space = 8 + 4 + 1 + 1)]
    pub settings: Account<'info, ElectionConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TallyVote<'info> {
    #[account(mut)]
    pub electorate_data: Account<'info, ElectorateData>,
    #[account(mut)]
    pub vote_data: Account<'info, VoteCount>,
    #[account(mut)]
    pub settings: Account<'info, ElectionConfig>,
    pub voter: Signer<'info>,
}

#[account]
pub struct ElectorateData {
    pub total_voters: u64,
    pub threshold: u64,
    pub is_active: bool,
}

#[account]
pub struct VoteCount {
    pub count: u32,
    pub last_vote: u64,
    pub bump: u8,
}

#[account]
pub struct ElectionConfig {
    pub threshold: u32,
    pub active: bool,
    pub bump: u8,
}


