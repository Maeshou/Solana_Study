// 6) art_palette_grant: パレット作成に伴う付与（細かい数値操作とループあり）
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use spl_token::instruction as token_ix;

declare_id!("ArtPa1ette6666666666666666666666666666666");

#[program]
pub mod art_palette_grant {
    use super::*;
    pub fn config(ctx: Context<Config>, seed: u64) -> Result<()> {
        let a = &mut ctx.accounts.art;
        a.curator = ctx.accounts.curator.key();
        a.seed = seed;
        a.counter = 0;
        a.granted = 0;
        Ok(())
    }

    pub fn grant(ctx: Context<Grant>, colors: u16) -> Result<()> {
        let a = &mut ctx.accounts.art;

        // 乱数っぽい微調整（固定演算）
        let mut weight = a.seed % 11;
        let mut n = 0;
        while n < 4 {
            weight = weight.saturating_add(3);
            n += 1;
        }

        let raw = (colors as u64).saturating_mul(weight);
        if raw < 5 {
            a.counter = a.counter.saturating_add(1);
            return Ok(());
        }

        let ix = token_ix::transfer(
            &ctx.accounts.any_program.key(),
            &ctx.accounts.pool.key(),
            &ctx.accounts.artist_vault.key(),
            &ctx.accounts.curator.key(),
            &[],
            raw,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.any_program.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.artist_vault.to_account_info(),
                ctx.accounts.curator.to_account_info(),
            ],
        )?;
        a.granted = a.granted.saturating_add(raw);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Config<'info> {
    #[account(init, payer = curator, space = 8 + 32 + 8 + 8 + 8)]
    pub art: Account<'info, Art>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Grant<'info> {
    #[account(mut, has_one = curator)]
    pub art: Account<'info, Art>,
    pub curator: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub artist_vault: UncheckedAccount<'info>,
    /// CHECK:
    pub any_program: UncheckedAccount<'info>,
}

#[account]
pub struct Art {
    pub curator: Pubkey,
    pub seed: u64,
    pub counter: u64,
    pub granted: u64,
}
