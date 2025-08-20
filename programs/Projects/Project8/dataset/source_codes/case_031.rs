use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ShopRegisterSafe11111111111111111111111111");

#[program]
pub mod shop_register_safe {
    use super::*;

    pub fn init_shop(ctx: Context<InitShop>, catalog: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.shop;
        s.owner = ctx.accounts.owner.key();
        s.catalog = catalog;
        s.salt = 41u64.rotate_left(2).wrapping_add(9);
        for k in 0..3u8 {
            let x = (s.salt ^ (k as u64 * 23)).rotate_left(1);
            s.salt = s.salt.wrapping_add(x).wrapping_mul(2).wrapping_add(5);
        }
        Ok(())
    }

    pub fn settle_refund(ctx: Context<SettleRefund>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.shop.key(), &ctx.accounts.customer.key(), lamports);

        let bump = *ctx.bumps.get("shop").ok_or(error!(ShopErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"shop",
            ctx.accounts.owner.key.as_ref(),
            ctx.accounts.shop.catalog.as_ref(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.shop.to_account_info(),
                ctx.accounts.customer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(RefundSettled { to: ctx.accounts.customer.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct RefundSettled {
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitShop<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 8,
        seeds = [b"shop", owner.key().as_ref(), catalog.key().as_ref()],
        bump
    )]
    pub shop: Account<'info, ShopState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: 実運用は適切な型で検証
    pub catalog: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleRefund<'info> {
    #[account(
        mut,
        seeds = [b"shop", owner.key().as_ref(), shop.catalog.key().as_ref()],
        bump
    )]
    pub shop: Account<'info, ShopState>,
    #[account(mut)]
    pub customer: SystemAccount<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ShopState {
    pub owner: Pubkey,
    pub catalog: Pubkey,
    pub salt: u64,
}

#[error_code]
pub enum ShopErr {
    #[msg("missing bump")] MissingBump,
}
