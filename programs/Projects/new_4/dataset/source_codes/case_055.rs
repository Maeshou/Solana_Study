// 10. クラウドファンディング＋寄付ログ
use anchor_lang::prelude::*;
declare_id!("CFND111122223333444455556666777788");

#[program]
pub mod misinit_crowd_v6 {
    use super::*;

    pub fn init_campaign(
        ctx: Context<InitCampaign>,
        goal: u64,
    ) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.goal = goal;
        c.raised = 0;
        Ok(())
    }

    pub fn contribute(
        ctx: Context<InitCampaign>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode10::InvalidAmt);
        let c = &mut ctx.accounts.campaign;
        c.raised = c.raised.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn record_contribution(
        ctx: Context<InitCampaign>,
        donor: Pubkey,
    ) -> Result<()> {
        let log = &mut ctx.accounts.contribution_log;
        log.donors.push(donor);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCampaign<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8)] pub campaign: Account<'info, CampaignData>,
    #[account(mut)] pub contribution_log: Account<'info, ContributionLog>,
    #[account(mut)] pub user: Signer<'info>, pub system_program: Program<'info, System>,
}

#[account]
pub struct CampaignData { pub goal:u64, pub raised:u64 }
#[account]
pub struct ContributionLog { pub donors: Vec<Pubkey> }

#[error_code]
pub enum ErrorCode10 { #[msg("寄付額は正の値である必要があります。")] InvalidAmt }
