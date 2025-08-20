// Program 2: store_coupon_payout
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("StorECouponPayout22222222222222222222222");

#[program]
pub mod store_coupon_payout {
    use super::*;

    pub fn init_store(ctx: Context<InitStore>, mint: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.store;
        s.merchant = ctx.accounts.merchant.key();
        s.mint = mint;
        s.score = 41u64.rotate_left(1).wrapping_add(23);
        for _ in 0..2 {
            s.score = s.score.rotate_right(2).wrapping_mul(3).wrapping_add(13);
        }
        Ok(())
    }

    pub fn payout(ctx: Context<Payout>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.store.key(), &ctx.accounts.customer.key(), amount);

        let bump = *ctx.bumps.get("store").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"store",
            ctx.accounts.merchant.key.as_ref(),
            ctx.accounts.store.mint.as_ref(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.store.to_account_info(),
                ctx.accounts.customer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStore<'info> {
    #[account(
        init,
        payer = merchant,
        space = 8 + 32 + 32 + 8,
        seeds=[b"store", merchant.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    /// CHECK: 実運用では Mint 型を推奨
    pub mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(
        mut,
        seeds=[b"store", merchant.key().as_ref(), store.mint.key().as_ref()],
        bump
    )]
    pub store: Account<'info, Store>,
    #[account(mut)]
    pub customer: SystemAccount<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Store {
    pub merchant: Pubkey,
    pub mint: Pubkey,
    pub score: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
