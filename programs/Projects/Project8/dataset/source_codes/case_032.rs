use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ArenaPoolSafe1111111111111111111111111111");

#[program]
pub mod arena_pool_safe {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, season: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.host = ctx.accounts.host.key();
        p.season = season.rotate_left(2).wrapping_add(12);
        p.stat = (p.season ^ 77).rotate_right(1);
        Ok(())
    }

    pub fn pay_prize(ctx: Context<PayPrize>, lamports: u64) -> Result<()> {
        let ix = system_instruction::transfer(&ctx.accounts.pool.key(), &ctx.accounts.champion.key(), lamports);

        let bump = *ctx.bumps.get("pool").ok_or(error!(ArenaErr::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"pool",
            ctx.accounts.host.key.as_ref(),
            &ctx.accounts.pool.season.to_le_bytes(),
            &[bump],
        ];

        invoke_signed(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.champion.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;

        emit!(PrizePaid { season: ctx.accounts.pool.season, to: ctx.accounts.champion.key(), amount: lamports });
        Ok(())
    }
}

#[event]
pub struct PrizePaid {
    pub season: u64,
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        payer = host,
        space = 8 + 32 + 8 + 8,
        seeds = [b"pool", host.key().as_ref(), season.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, ArenaPool>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub season: u64,
}

#[derive(Accounts)]
pub struct PayPrize<'info> {
    #[account(
        mut,
        seeds = [b"pool", host.key().as_ref(), pool.season.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, ArenaPool>,
    #[account(mut)]
    pub champion: SystemAccount<'info>,
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ArenaPool {
    pub host: Pubkey,
    pub season: u64,
    pub stat: u64,
}

#[error_code]
pub enum ArenaErr {
    #[msg("missing bump")] MissingBump,
}
