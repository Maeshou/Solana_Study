use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

declare_id!("EqpExa7777777777777777777777777777777777");

#[program]
pub mod secure_crowdfunding {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        title: String,
        description: String,
        goal_amount: u64,
        duration_days: u32,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        campaign.creator = ctx.accounts.creator.key();
        campaign.title = title;
        campaign.description = description;
        campaign.goal_amount = goal_amount;
        campaign.raised_amount = 0;
        campaign.start_time = current_time;
        campaign.end_time = current_time
            .checked_add(duration_days as i64 * 24 * 60 * 60)
            .ok_or(CrowdfundingError::Overflow)?;
        campaign.is_active = true;
        campaign.is_successful = false;
        campaign.backer_count = 0;
        campaign.bump = *ctx.bumps.get("campaign").unwrap();
        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let contribution = &mut ctx.accounts.contribution;
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Validations
        require!(campaign.is_active, CrowdfundingError::CampaignNotActive);
        require!(current_time <= campaign.end_time, CrowdfundingError::CampaignEnded);
        require!(amount > 0, CrowdfundingError::InvalidAmount);

        // Initialize contribution account
        contribution.campaign = campaign.key();
        contribution.backer = ctx.accounts.backer.key();
        contribution.amount = amount;
        contribution.contributed_at = current_time;
        contribution.refunded = false;
        contribution.bump = *ctx.bumps.get("contribution").unwrap();

        // Update campaign state
        campaign.raised_amount = campaign
            .raised_amount
            .checked_add(amount)
            .ok_or(CrowdfundingError::Overflow)?;
        campaign.backer_count = campaign
            .backer_count
            .checked_add(1)
            .ok_or(CrowdfundingError::Overflow)?;

        if campaign.raised_amount >= campaign.goal_amount {
            campaign.is_successful = true;
        }

        // Transfer funds to campaign escrow
        let ix = system_instruction::transfer(
            &ctx.accounts.backer.key(),
            &ctx.accounts.campaign.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.backer.to_account_info(),
                ctx.accounts.campaign.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn finalize_campaign(ctx: Context<FinalizeCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        require!(current_time > campaign.end_time, CrowdfundingError::CampaignStillActive);
        require!(campaign.creator == ctx.accounts.creator.key(), CrowdfundingError::Unauthorized);

        campaign.is_active = false;

        if campaign.is_successful {
            let amount = campaign.raised_amount;
            // Transfer to creator
            let ix = system_instruction::transfer(
                &campaign.key(),
                &ctx.accounts.creator.key(),
                amount,
            );
            invoke(
                &ix,
                &[
                    ctx.accounts.campaign.to_account_info(),
                    ctx.accounts.creator.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }
        Ok(())
    }

    pub fn refund_contribution(ctx: Context<RefundContribution>) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let contribution = &mut ctx.accounts.contribution;

        require!(!campaign.is_active, CrowdfundingError::CampaignStillActive);
        require!(!campaign.is_successful, CrowdfundingError::CampaignSuccessful);
        require!(contribution.backer == ctx.accounts.backer.key(), CrowdfundingError::Unauthorized);
        require!(!contribution.refunded, CrowdfundingError::AlreadyRefunded);

        contribution.refunded = true;
        let amount = contribution.amount;

        // Refund the contribution
        let ix = system_instruction::transfer(
            &ctx.accounts.campaign.key(),
            &ctx.accounts.backer.key(),
            amount,
        );
        invoke(
            &ix,
            &[
                ctx.accounts.campaign.to_account_info(),
                ctx.accounts.backer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Campaign {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub goal_amount: u64,
    pub raised_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_active: bool,
    pub is_successful: bool,
    pub backer_count: u64,
    pub bump: u8,
}

#[account]
pub struct Contribution {
    pub campaign: Pubkey,
    pub backer: Pubkey,
    pub amount: u64,
    pub contributed_at: i64,
    pub refunded: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = creator,
        space = 8  // discriminator
            + 32  // creator
            + 4 + title.as_bytes().len()
            + 4 + description.as_bytes().len()
            + 8  // goal_amount
            + 8  // raised_amount
            + 8  // start_time
            + 8  // end_time
            + 1  // is_active
            + 1  // is_successful
            + 8  // backer_count
            + 1  // bump
        ,
        seeds = [b"campaign", creator.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref(), campaign.title.as_bytes()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = backer,
        space = 8 + 32 + 32 + 8 + 8 + 1 + 1,
        seeds = [b"contribution", campaign.key().as_ref(), backer.key().as_ref()],
        bump
    )]
    pub contribution: Account<'info, Contribution>,

    #[account(mut)]
    pub backer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeCampaign<'info> {
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref(), campaign.title.as_bytes()],
        bump = campaign.bump,
        constraint = campaign.creator == creator.key(),
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefundContribution<'info> {
    #[account(
        mut,
        seeds = [b"campaign", campaign.creator.as_ref(), campaign.title.as_bytes()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        mut,
        seeds = [b"contribution", campaign.key().as_ref(), backer.key().as_ref()],
        bump = contribution.bump,
        constraint = contribution.backer == backer.key(),
    )]
    pub contribution: Account<'info, Contribution>,

    #[account(mut)]
    pub backer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CrowdfundingError {
    #[msg("Campaign is not active")]
    CampaignNotActive,
    #[msg("Campaign has ended")]
    CampaignEnded,
    #[msg("Invalid contribution amount")]
    InvalidAmount,
    #[msg("Campaign is still active")]
    CampaignStillActive,
    #[msg("Campaign was successful, no refunds")]
    CampaignSuccessful,
    #[msg("Unauthorized")]        
    Unauthorized,
    #[msg("Arithmetic overflow")] 
    Overflow,
    #[msg("Index out of range")]
    IndexOutOfRange,
    #[msg("New ID invalid")]
    InvalidNewId,
    #[msg("Already refunded")]
    AlreadyRefunded,
}
