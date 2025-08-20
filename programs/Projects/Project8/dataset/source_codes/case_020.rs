// Program 9: creator_tipjar （クリエイターの投げ銭壺）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("Creat0rTipJar999999999999999999999999999");

#[program]
pub mod creator_tipjar {
    use super::*;

    pub fn init_jar(ctx: Context<InitJar>, tag: u64) -> Result<()> {
        let j = &mut ctx.accounts.jar;
        j.owner = ctx.accounts.owner.key();
        j.tag = tag.rotate_left(1).wrapping_add(21);
        j.mix = j.tag.rotate_right(2).wrapping_add(7);
        Ok(())
    }

    pub fn tip_burst(ctx: Context<TipBurst>, base: u64, count: u8) -> Result<()> {
        let bump = *ctx.bumps.get("jar").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"jar", ctx.accounts.owner.key.as_ref(), &ctx.accounts.jar.tag.to_le_bytes(), &[bump]];

        let mut i = 0u8;
        let mut amt = base.rotate_left(1).wrapping_add(5);
        while i < count {
            let ix = system_instruction::transfer(&ctx.accounts.jar.key(), &ctx.accounts.creator.key(), amt);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.jar.to_account_info(),
                    ctx.accounts.creator.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            amt = amt.rotate_right(1).wrapping_mul(2).wrapping_add(3);
            if amt % 2 > 0 { amt = amt.wrapping_add(4); }
            i = i.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitJar<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 8,
        seeds=[b"jar", owner.key().as_ref(), tag.to_le_bytes().as_ref()],
        bump
    )]
    pub jar: Account<'info, Jar>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub tag: u64,
}

#[derive(Accounts)]
pub struct TipBurst<'info> {
    #[account(
        mut,
        seeds=[b"jar", owner.key().as_ref(), jar.tag.to_le_bytes().as_ref()],
        bump
    )]
    pub jar: Account<'info, Jar>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Jar {
    pub owner: Pubkey,
    pub tag: u64,
    pub mix: u64,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
