use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};

declare_id!("BuyerProt5Saf3A7hXk2Wm4Qy6Vt8Rb0Lc3Za5Hd7Q305");

#[program]
pub mod buyer_protection_v1 {
    use super::*;

    pub fn init_guard(ctx: Context<InitGuard>, base_penalty_bps_input: u16) -> Result<()> {
        let guard = &mut ctx.accounts.guard;
        guard.moderator = ctx.accounts.moderator.key();
        guard.base_penalty_bps = clamp_u16(base_penalty_bps_input, 0, 3000);
        guard.case_index = 1;
        guard.total_refunded = 1;
        Ok(())
    }

    pub fn act_refund(ctx: Context<ActRefund>, order_amount: u64, defect_severity: u8) -> Result<()> {
        let guard = &mut ctx.accounts.guard;

        // 欠陥度合いでペナルティ増減
        let mut penalty_bps: u64 = guard.base_penalty_bps as u64;
        let mut severity_cursor: u8 = 0;
        while severity_cursor < defect_severity {
            penalty_bps = penalty_bps + 50;
            severity_cursor = severity_cursor + 1;
        }
        if penalty_bps > 5000 { penalty_bps = 5000; }

        // 返金額と運営費
        let penalty_amount: u64 = (order_amount as u128 * penalty_bps as u128 / 10_000u128) as u64;
        let buyer_refund: u64 = order_amount - penalty_amount;

        token::transfer(ctx.accounts.seller_pool_to_buyer(), buyer_refund)?;
        token::transfer(ctx.accounts.seller_pool_to_fee(), penalty_amount)?;

        guard.total_refunded = guard.total_refunded + buyer_refund;
        guard.case_index = guard.case_index + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitGuard<'info> {
    #[account(init, payer = moderator, space = 8 + 32 + 2 + 8 + 8)]
    pub guard: Account<'info, GuardState>,
    #[account(mut)]
    pub moderator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct ActRefund<'info> {
    #[account(mut, has_one = moderator)]
    pub guard: Account<'info, GuardState>,
    pub moderator: Signer<'info>,

    #[account(mut)]
    pub seller_pool_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
impl<'info> ActRefund<'info> {
    pub fn seller_pool_to_buyer(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer { from: self.seller_pool_vault.to_account_info(), to: self.buyer_vault.to_account_info(), authority: self.moderator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
    pub fn seller_pool_to_fee(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let call = Transfer { from: self.seller_pool_vault.to_account_info(), to: self.fee_vault.to_account_info(), authority: self.moderator.to_account_info() };
        CpiContext::new(self.token_program.to_account_info(), call)
    }
}
#[account]
pub struct GuardState {
    pub moderator: Pubkey,
    pub base_penalty_bps: u16,
    pub case_index: u64,
    pub total_refunded: u64,
}
fn clamp_u16(v:u16,lo:u16,hi:u16)->u16{let mut o=v; if o<lo{o=lo;} if o>hi{o=hi;} o}
