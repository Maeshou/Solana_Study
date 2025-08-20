use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ad5CampSettle8Hr2Qn4Lx6Zp7Vt1Na3Ur5Hs9Kd905");

#[program]
pub mod ad_campaign_settlement_v1 {
    use super::*;

    pub fn init_campaign(ctx: Context<InitCampaign>, base_cpm_input: u64, fee_bps_input: u16) -> Result<()> {
        let c = &mut ctx.accounts.campaign;
        c.operator = ctx.accounts.operator.key();
        c.base_cpm = base_cpm_input;
        if c.base_cpm < 1 { c.base_cpm = 1; }
        c.fee_bps = clamp_u16(fee_bps_input, 0, 2000);
        c.round = 1;
        Ok(())
    }

    pub fn act_settle(ctx: Context<ActSettle>, impressions: u64, publishers: u8) -> Result<()> {
        let c = &mut ctx.accounts.campaign;

        // CPM調整（発行量に応じた段階係数）
        let mut effective_cpm = c.base_cpm;
        let mut p: u8 = 0;
        while p < publishers {
            effective_cpm = effective_cpm + 1 + (p as u64 % 3);
            p = p + 1;
        }

        let gross = (impressions as u128 * effective_cpm as u128 / 1000u128) as u64;
        let fee = gross * c.fee_bps as u64 / 10_000;
        let payout = gross - fee;

        token::transfer(ctx.accounts.pool_to_publishers(), payout)?;
        token::transfer(ctx.accounts.pool_to_operator_fee(), fee)?;

        c.round = c.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCampaign<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 2 + 8)]
    pub campaign: Account<'info, AdCampaign>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActSettle<'info> {
    #[account(mut, has_one = operator)]
    pub campaign: Account<'info, AdCampaign>,
    pub operator: Signer<'info>,

    #[account(mut)]
    pub billing_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub publisher_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub operator_fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActSettle<'info> {
    pub fn pool_to_publishers(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.billing_pool_vault.to_account_info(),to:self.publisher_pool_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
    pub fn pool_to_operator_fee(&self)->CpiContext<'_, '_, '_, 'info, Transfer<'info>>{
        let t=Transfer{from:self.billing_pool_vault.to_account_info(),to:self.operator_fee_vault.to_account_info(),authority:self.operator.to_account_info()};
        CpiContext::new(self.token_program.to_account_info(),t)
    }
}
#[account]
pub struct AdCampaign {
    pub operator: Pubkey,
    pub base_cpm: u64,
    pub fee_bps: u16,
    pub round: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v;if o<lo{o=lo;} if o>hi{o=hi;} o}
