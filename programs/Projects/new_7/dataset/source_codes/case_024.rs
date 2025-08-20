// 7) raid_bounty_stream
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Ra1dB0untyStr3am000000000000000000000007");

#[program]
pub mod raid_bounty_stream {
    use super::*;

    pub fn spawn(ctx: Context<Spawn>, base: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.master = ctx.accounts.master.key();
        b.base = base;
        b.ok = 0;
        b.ng = 0;
        b.track = 0;

        let mut k = 0u8;
        while k < 5 {
            b.track = b.track.saturating_add((k as u32) + 1);
            k = k.saturating_add(1);
        }
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, weight: u8, proof: String) -> Result<()> {
        let b = &mut ctx.accounts.board;
        require!(b.master == ctx.accounts.master.key(), Errs::Master);

        if proof.len() > 8 {
            let mut sum = 0u64;
            let bytes = proof.as_bytes();
            let mut i = 0usize;
            while i < bytes.len() {
                sum = sum.saturating_add((bytes[i] as u64) % 13 + 1);
                if i % 5 == 0 { b.ok = b.ok.saturating_add(1); }
                i += 1;
            }
            b.track = b.track.saturating_add((sum % 11) as u32);
        } else {
            let mut r = 0u8;
            while r < 7 {
                if b.ng < 1_000_000 { b.ng = b.ng.saturating_add(1); }
                if r % 2 == 0 && b.track > 0 { b.track = b.track.saturating_sub(1); }
                r = r.saturating_add(1);
            }
        }

        let mut amt = b.base.saturating_mul(weight as u64);
        let mut lift = 0u64;
        let mut z = 0u8;
        while z < 6 {
            lift = lift.saturating_add((z as u64) + ((b.ok % 5) as u64));
            z = z.saturating_add(1);
        }
        amt = amt.saturating_add(lift);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.treasury.key(),
            ctx.accounts.hunter_ata.key(),
            ctx.accounts.master.key(),
            &[],
            amt,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.hunter_ata.to_account_info(),
                ctx.accounts.master.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Board {
    pub master: Pubkey,
    pub base: u64,
    pub ok: u32,
    pub ng: u32,
    pub track: u32,
}

#[derive(Accounts)]
pub struct Spawn<'info> {
    #[account(init, payer = master, space = 8 + 32 + 8 + 4 + 4 + 4)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    pub master: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub hunter_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("master mismatch")] Master }
