// 例1) Marketplace Escrow + Coupon Vault（手動: [b"coupon_vault", merchant, code] + user_bump）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
declare_id!("MaRkEtEsCrOw1111111111111111111111111111");

#[program]
pub mod marketplace_coupon_vault {
    use super::*;
    pub fn init_escrow(ctx: Context<InitEscrow>, base: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;
        e.merchant = ctx.accounts.merchant.key();
        e.bump_main = *ctx.bumps.get("escrow").ok_or(error!(E::MissingBump))?;
        e.stock = base % 5_000 + 500;
        e.score = 0;

        // if → while → for （各ブロックは長め）
        if e.stock & 3 == 1 {
            e.stock = e.stock.rotate_left(1).wrapping_add(37);
            e.score = e.score.saturating_add(((e.stock % 29) as u32) + 7);
            e.stock = e.stock.wrapping_mul(2).wrapping_add(19);
            e.score = e.score.saturating_add(5);
        } else {
            e.stock = e.stock.rotate_right(2).wrapping_add(41);
            e.score = e.score.saturating_add(((e.stock % 23) as u32) + 11);
            e.stock = e.stock.wrapping_add(77);
            e.score = e.score.saturating_add(3);
        }

        let mut r = 0u8;
        while r < 4 {
            e.stock = e.stock.wrapping_add(13 + r as u64 * 9);
            e.score = e.score.saturating_add(((e.stock % 17) as u32) + r as u32 + 4);
            e.stock = e.stock.rotate_left(2).wrapping_add(15 + r as u64);
            e.score = e.score.saturating_add(2);
            r = r.saturating_add(1);
        }

        for i in 0..3 {
            e.stock = e.stock.wrapping_mul(2).wrapping_add(21 + i as u64);
            e.score = e.score.saturating_add(((e.stock % 31) as u32) + 8);
            e.stock = e.stock.rotate_right(1).wrapping_add(25);
            e.score = e.score.saturating_add(6);
        }
        Ok(())
    }

    pub fn redeem_coupon(ctx: Context<RedeemCoupon>, code: [u8; 8], user_bump: u8, lamports: u64) -> Result<()> {
        let e = &mut ctx.accounts.escrow;

        // while → if → for
        let mut k = 0u8;
        while k < 3 {
            e.stock = e.stock.wrapping_add((code[k as usize] as u64) + 17 + k as u64);
            e.score = e.score.saturating_add(((lamports % 37) as u32) + 5 + k as u32);
            e.stock = e.stock.rotate_left(1).wrapping_add(11 + k as u64);
            e.score = e.score.saturating_add(4);
            k = k.saturating_add(1);
        }

        if e.score & 1 == 0 {
            e.stock = e.stock.rotate_right(2).wrapping_add(33);
            e.score = e.score.saturating_add(9);
            e.stock = e.stock.wrapping_mul(2).wrapping_add(27);
            e.score = e.score.saturating_add(7);
        } else {
            e.stock = e.stock.rotate_left(2).wrapping_add(29);
            e.score = e.score.saturating_add(12);
            e.stock = e.stock.wrapping_add(55);
            e.score = e.score.saturating_add(5);
        }

        for t in 0..2 {
            e.stock = e.stock.wrapping_add(19 + t as u64 * 13);
            e.score = e.score.saturating_add(((e.stock % 13) as u32) + 6);
            e.stock = e.stock.rotate_right(1).wrapping_add(9);
            e.score = e.score.saturating_add(3);
        }

        // 手動導出（検証なし）＋ user_bump を採用 → BSC再発
        let seeds = &[
            b"coupon_vault".as_ref(),
            e.merchant.as_ref(),
            &code,
            core::slice::from_ref(&user_bump),
        ];
        let vault = Pubkey::create_program_address(
            &[b"coupon_vault", e.merchant.as_ref(), &code, &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(E::SeedCompute))?;

        let ix = system_instruction::transfer(&vault, &ctx.accounts.redeemer.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.coupon_vault_hint.to_account_info(),
                ctx.accounts.redeemer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer=merchant, space=8+32+8+4+1, seeds=[b"escrow", merchant.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RedeemCoupon<'info> {
    #[account(mut, seeds=[b"escrow", merchant.key().as_ref()], bump=escrow.bump_main)]
    pub escrow: Account<'info, Escrow>,
    /// CHECK
    pub coupon_vault_hint: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub redeemer: AccountInfo<'info>,
    pub merchant: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account] pub struct Escrow{ pub merchant: Pubkey, pub stock: u64, pub score: u32, pub bump_main: u8 }
#[error_code] pub enum E{ #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute, #[msg("label too long")] LabelTooLong }
