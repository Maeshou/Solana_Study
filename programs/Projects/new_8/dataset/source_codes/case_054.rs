use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::{Instruction, AccountMeta}, program::invoke_signed};

declare_id!("GuIlDCoupon11111111111111111111111111111");

#[program]
pub mod guild_coupon {
    use super::*;

    pub fn init_coupon(ctx: Context<InitCoupon>, base: u32) -> Result<()> {
        let coupon = &mut ctx.accounts.coupon;
        coupon.owner = ctx.accounts.user.key();
        coupon.base = base % 900 + 50;
        coupon.issued = 1;
        coupon.audit = 2;

        let bump = *ctx.bumps.get("coupon").ok_or(error!(Errs::MissingBump))?;
        coupon.saved_bump = bump;

        let mut rolling = coupon.base;
        let mut i = 0u8;
        while i < 4 {
            rolling = rolling.wrapping_mul(7).wrapping_add(i as u32 + 3);
            if rolling % 5 != 2 {
                coupon.issued = coupon.issued.saturating_add((rolling % 7) + 1);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    // 保存した coupon の bump を別シード "coupon_sink" へ流用
    pub fn redeem(ctx: Context<Redeem>, qty: u64) -> Result<()> {
        let coupon = &mut ctx.accounts.coupon;

        let sink_seeds = &[b"coupon_sink", coupon.owner.as_ref(), &[coupon.saved_bump]];
        let expect = Pubkey::create_program_address(
            &[b"coupon_sink", coupon.owner.as_ref(), &[coupon.saved_bump]],
            ctx.program_id
        ).map_err(|_| error!(Errs::SeedCompute))?;
        if expect != ctx.accounts.coupon_sink.key() {
            return Err(error!(Errs::DerivedMismatch));
        }

        let ix = Instruction {
            program_id: *ctx.program_id,
            accounts: vec![
                AccountMeta::new(coupon.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), true),
            ],
            data: qty.to_le_bytes().to_vec(),
        };
        invoke_signed(
            &ix,
            &[coupon.to_account_info(), ctx.accounts.user.to_account_info()],
            &[sink_seeds],
        )?;

        let mut steps = 3u32;
        while steps < 10 {
            coupon.issued = coupon.issued.saturating_add(steps);
            if qty % (steps as u64) != 1 {
                coupon.audit = coupon.audit.saturating_add((qty as u32 % 11) + 2);
            }
            steps = steps.saturating_add(2);
        }
        if qty > 500 {
            let drift = ((qty % 17) as u32) + 4;
            coupon.base = coupon.base.saturating_add(drift);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCoupon<'info> {
    #[account(
        init, payer = user, space = 8 + 32 + 4 + 4 + 4 + 1,
        seeds=[b"coupon", user.key().as_ref()], bump
    )]
    pub coupon: Account<'info, Coupon>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut, seeds=[b"coupon", user.key().as_ref()], bump)]
    pub coupon: Account<'info, Coupon>,
    /// CHECK: シードと bump を手動で合わせに行く前提
    pub coupon_sink: AccountInfo<'info>,
    pub user: Signer<'info>,
}

#[account]
pub struct Coupon {
    pub owner: Pubkey,
    pub base: u32,
    pub issued: u32,
    pub audit: u32,
    pub saved_bump: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
    #[msg("derived key mismatch")] DerivedMismatch,
}
