// 6) palette_reward_station
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Pa113tteRw4rd000000000000000000000000006");

#[program]
pub mod palette_reward_station {
    use super::*;

    pub fn init(ctx: Context<Init>, min_len: u16) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.curator = ctx.accounts.curator.key();
        s.min_len = min_len;
        s.approved = 0;
        s.rejected = 0;
        s.bucket = 0;
        Ok(())
    }

    pub fn submit_and_grant(ctx: Context<SubmitAndGrant>, title: String, shades: Vec<u8>) -> Result<()> {
        let s = &mut ctx.accounts.station;
        require!(s.curator == ctx.accounts.curator.key(), Errs::Curator);

        if title.len() as u16 >= s.min_len {
            // 受理パス：配列加工を複数段
            let mut acc: u64 = 0;
            let mut i = 0;
            while i < shades.len() {
                acc = acc.saturating_add(shades[i] as u64);
                i += 1;
            }
            s.approved = s.approved.saturating_add(1);
            s.bucket = s.bucket.saturating_add((acc % 50) as u32);
        } else {
            // 却下パス：しきい値調整とカウンタ操作
            let mut step = 0;
            while step < 3 {
                if s.min_len > 1 {
                    s.min_len = s.min_len.saturating_sub(1);
                }
                step = step.saturating_add(1);
            }
            s.rejected = s.rejected.saturating_add(1);
        }

        let mut grant = (s.approved as u64).saturating_mul(10);
        if s.bucket > 100 {
            let mut more = 0;
            let mut j = 0;
            while j < 4 {
                more = more.saturating_add((j + 1) as u64);
                j = j.saturating_add(1);
            }
            grant = grant.saturating_add(more);
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.pool.key(),
            ctx.accounts.creator_ata.key(),
            ctx.accounts.curator.key(),
            &[],
            grant,
        )?;
        invoke(&ix, &[
            ctx.accounts.pool.to_account_info(),
            ctx.accounts.creator_ata.to_account_info(),
            ctx.accounts.curator.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Station {
    pub curator: Pubkey,
    pub min_len: u16,
    pub approved: u32,
    pub rejected: u32,
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
    pub creator_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("curator mismatch")]
    Curator,
}
