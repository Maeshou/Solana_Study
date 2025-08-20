use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("Ref7erralSp1litG8E2O9M5A1Z6X4C7V0T3Y707");

#[program]
pub mod referral_split_v1 {
    use super::*;

    pub fn init_deal(ctx: Context<InitDeal>, base_bps: u16) -> Result<()> {
        let deal = &mut ctx.accounts.deal;
        deal.manager = ctx.accounts.manager.key();
        deal.base_bps = clamp_u16(base_bps, 500, 9000);
        deal.depth_limit = 2;
        deal.round = 1;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<ActDistribute>, revenue_units: u64) -> Result<()> {
        let deal = &mut ctx.accounts.deal;

        // 幾何減衰（レベルごとに半減）
        let level1_bps = deal.base_bps as u64;
        let level2_bps = level1_bps / 2;

        let l1_share = revenue_units * level1_bps / 10_000;
        let l2_share = revenue_units * level2_bps / 10_000;
        let remainder = revenue_units - l1_share - l2_share;

        token::transfer(ctx.accounts.revenue_to_level1(), l1_share)?;
        token::transfer(ctx.accounts.revenue_to_level2(), l2_share)?;
        token::transfer(ctx.accounts.revenue_to_manager(), remainder)?;

        deal.round = deal.round + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDeal<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 2 + 1 + 8)]
    pub deal: Account<'info, ReferralDeal>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActDistribute<'info> {
    #[account(mut, has_one = manager)]
    pub deal: Account<'info, ReferralDeal>,
    pub manager: Signer<'info>,

    #[account(mut)]
    pub revenue_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub referrer1_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub referrer2_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActDistribute<'info> {
    pub fn revenue_to_level1(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from: self.revenue_vault.to_account_info(), to: self.referrer1_vault.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn revenue_to_level2(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from: self.revenue_vault.to_account_info(), to: self.referrer2_vault.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
    pub fn revenue_to_manager(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let c = Transfer { from: self.revenue_vault.to_account_info(), to: self.manager.to_account_info(), authority: self.manager.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), c)
    }
}

#[account]
pub struct ReferralDeal {
    pub manager: Pubkey,
    pub base_bps: u16,
    pub depth_limit: u8,
    pub round: u64,
}

fn clamp_u16(v: u16, lo: u16, hi: u16) -> u16 { let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o }
