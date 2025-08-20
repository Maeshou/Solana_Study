// Program 6: subscription_hub （サブスクリプション集金）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("SubsCriptionHub6666666666666666666666666");

#[program]
pub mod subscription_hub {
    use super::*;

    pub fn init_hub(ctx: Context<InitHub>, plan: u64) -> Result<()> {
        let h = &mut ctx.accounts.hub;
        h.owner = ctx.accounts.owner.key();
        h.plan = plan.rotate_left(1).wrapping_add(27);
        h.tick = 1;
        let mut i = 0u8;
        let mut r = h.plan.rotate_right(1).wrapping_add(9);
        while i < 3 {
            r = r.rotate_left(1).wrapping_mul(2).wrapping_add(5);
            if r & 1 > 0 { h.tick = h.tick.saturating_add(((r % 17) as u32) + 2); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn collect(ctx: Context<Collect>, price: u64, cycles: u8) -> Result<()> {
        let bump = *ctx.bumps.get("hub").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"hub", ctx.accounts.owner.key.as_ref(), &ctx.accounts.hub.plan.to_le_bytes(), &[bump]];

        let mut i = 0u8;
        let mut amt = price.rotate_left(2).wrapping_add(3);
        while i < cycles {
            let ix = system_instruction::transfer(&ctx.accounts.hub.key(), &ctx.accounts.seller.key(), amt);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.hub.to_account_info(),
                    ctx.accounts.seller.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            amt = amt.rotate_right(1).wrapping_mul(3).wrapping_add(11);
            if amt % 3 > 0 { amt = amt.wrapping_add(7); }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHub<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 4,
        seeds=[b"hub", owner.key().as_ref(), plan.to_le_bytes().as_ref()],
        bump
    )]
    pub hub: Account<'info, Hub>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub plan: u64,
}

#[derive(Accounts)]
pub struct Collect<'info> {
    #[account(
        mut,
        seeds=[b"hub", owner.key().as_ref(), hub.plan.to_le_bytes().as_ref()],
        bump
    )]
    pub hub: Account<'info, Hub>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Hub {
    pub owner: Pubkey,
    pub plan: u64,
    pub tick: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
