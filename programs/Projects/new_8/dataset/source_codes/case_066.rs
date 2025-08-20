// 例1) シード順序の入れ替え（検証: [b"chest", owner] / 署名: [owner, b"chest"]）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ChEsTOrdEr111111111111111111111111111111");

#[program]
pub mod chest_router {
    use super::*;

    pub fn init(ctx: Context<Init>, seed_tweak: u64) -> Result<()> {
        let chest = &mut ctx.accounts.chest;
        chest.owner = ctx.accounts.owner.key();
        chest.bump_saved = *ctx.bumps.get("chest").ok_or(error!(Errs::MissingBump))?;
        chest.score = seed_tweak.rotate_left(3).wrapping_add(77);
        let mut i = 0u8;
        while i < 5 {
            chest.score = chest.score.wrapping_mul(3).wrapping_add(i as u64 + 11);
            if chest.score % 2 == 1 {
                chest.score = chest.score.rotate_right(2).wrapping_add(19);
            } else {
                chest.score = chest.score.rotate_left(1).wrapping_add(23);
            }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn payout(ctx: Context<Payout>, lamports: u64) -> Result<()> {
        let c = &ctx.accounts.chest;
        // ❌ 検証は [b"chest", owner]、署名は [owner, b"chest"] に入れ替え
        let wrong_seeds: &[&[u8]] = &[c.owner.as_ref(), b"chest", &[c.bump_saved]];
        let derived = Pubkey::create_program_address(
            &[c.owner.as_ref(), b"chest", &[c.bump_saved]],
            ctx.program_id,
        ).map_err(|_| error!(Errs::SeedCompute))?;
        let ix = system_instruction::transfer(&derived, &ctx.accounts.receiver.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.derived_info.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[wrong_seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer=owner, space=8+32+8+1, seeds=[b"chest", owner.key().as_ref()], bump)]
    pub chest: Account<'info, Chest>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
    #[account(mut, seeds=[b"chest", owner.key().as_ref()], bump)]
    pub chest: Account<'info, Chest>,
    /// CHECK: 派生先のアカウントを「検証とは別シード」で署名しようとしている
    pub derived_info: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Chest {
    pub owner: Pubkey,
    pub score: u64,
    pub bump_saved: u8,
}

#[error_code]
pub enum Errs {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
}
