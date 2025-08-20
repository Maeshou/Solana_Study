// 05. マーケットの決済（手数料振替）
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("8bC7B6a5F4e3D2c1B0a9v8u7t6s5r4q3p2o1n0m9l8k7j6i5h4g3f2e1d0C9B8A7");

#[program]
pub mod marketplace_fee_manager {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee_bps: u16) -> Result<()> {
        let market_state = &mut ctx.accounts.marketplace_state;
        market_state.admin = ctx.accounts.admin.key();
        market_state.fee_bps = fee_bps;
        market_state.fee_collector_account = ctx.accounts.fee_collector_account.key();
        Ok(())
    }

    pub fn process_sale(ctx: Context<ProcessSale>, sale_amount: u64) -> Result<()> {
        let market_state = &ctx.accounts.marketplace_state;
        let fee_amount = sale_amount * market_state.fee_bps as u64 / 10000;
        let final_seller_amount = sale_amount - fee_amount;
        let mut paid_fee = false;

        if fee_amount > 0 {
            // Pay marketplace fee
            let fee_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.fee_collector_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            );
            token::transfer(fee_ctx, fee_amount)?;
            paid_fee = true;
        }

        if final_seller_amount > 0 {
            // Pay seller
            let seller_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.seller_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            );
            token::transfer(seller_ctx, final_seller_amount)?;
        } else {
            if !paid_fee {
                return Err(ErrorCode::NoPaymentMade.into());
            }
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(fee_bps: u16)]
pub struct InitializeMarketplace<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 32)]
    pub marketplace_state: Account<'info, MarketplaceState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub fee_collector_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProcessSale<'info> {
    #[account(has_one = admin)]
    pub marketplace_state: Account<'info, MarketplaceState>,
    #[account(mut, has_one = owner)]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    /// CHECK: This account is checked by the marketplace_state.
    #[account(mut)]
    pub fee_collector_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct MarketplaceState {
    pub admin: Pubkey,
    pub fee_bps: u16, // basis points
    pub fee_collector_account: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No payment was made, check the sale amount.")]
    NoPaymentMade,
}