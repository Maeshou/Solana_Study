// 4) painter_grant_pool
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Pa1nt3rGrantP00l000000000000000000000004");

#[program]
pub mod painter_grant_pool {
    use super::*;

    pub fn init(ctx: Context<Init>, base: u16) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.curator = ctx.accounts.curator.key();
        s.hurdle = base;
        s.accept = 0;
        s.reject = 0;
        s.bucket = 0;

        // 初期ゆらぎ
        let mut i = 0u8;
        while i < 6 {
            if i % 2 == 0 { s.bucket = s.bucket.saturating_add(3); }
            else if s.bucket > 0 { s.bucket = s.bucket.saturating_sub(1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn submit_and_grant(ctx: Context<SubmitAndGrant>, title: String, shades: Vec<u8>) -> Result<()> {
        let s = &mut ctx.accounts.station;
        require!(s.curator == ctx.accounts.curator.key(), Errs::Curator);

        if title.len() as u16 >= s.hurdle {
            // 受理：配列集計＋段階補正＋ハードル再設定
            let mut acc: u64 = 0;
            let mut p = 0usize;
            while p < shades.len() {
                acc = acc.saturating_add((shades[p] as u64) % 17 + 1);
                if p % 3 == 0 { s.bucket = s.bucket.saturating_add(1); }
                p += 1;
            }
            s.accept = s.accept.saturating_add(1);

            let mut t = 0u8;
            while t < 5 {
                s.hurdle = s.hurdle.saturating_add(1);
                if s.hurdle > 256 { s.hurdle = 256; }
                t = t.saturating_add(1);
            }
            s.bucket = s.bucket.saturating_add((acc % 50) as u32);
        } else {
            // 却下：段階的にしきい値を下げ、バケツを整える
            let mut d = 0u8;
            while d < 7 {
                if s.hurdle > 1 {
                    s.hurdle = s.hurdle.saturating_sub(1);
                }
                if d % 2 == 0 && s.bucket > 0 {
                    s.bucket = s.bucket.saturating_sub(1);
                }
                d = d.saturating_add(1);
            }
            s.reject = s.reject.saturating_add(1);
        }

        let mut grant = (s.accept as u64).saturating_mul(12);
        let mut spread = 0u64;
        let mut z = 0u8;
        while z < 4 {
            spread = spread.saturating_add(((s.bucket % 9) as u64) + (z as u64));
            z = z.saturating_add(1);
        }
        grant = grant.saturating_add(spread);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.pool.key(),
            ctx.accounts.artist_ata.key(),
            ctx.accounts.curator.key(),
            &[],
            grant,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.artist_ata.to_account_info(),
                ctx.accounts.curator.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Station {
    pub curator: Pubkey,
    pub hurdle: u16,
    pub accept: u32,
    pub reject: u32,
    pub bucket: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = curator, space = 8 + 32 + 2 + 4 + 4 + 4)]
    pub station: Account<'info, Station>,
    #[account(mut)]
    pub curator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SubmitAndGrant<'info> {
    #[account(mut)]
    pub station: Account<'info, Station>,
    pub curator: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub artist_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("curator mismatch")] Curator }
