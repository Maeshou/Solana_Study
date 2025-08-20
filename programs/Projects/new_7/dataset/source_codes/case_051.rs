// 5) prefix_suffix_mixer — Prefix/Suffix和を混合、状態IDを閾値で切替
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("Pr3fixSuff1xMix5r55555555555555555555555");

#[program]
pub mod prefix_suffix_mixer {
    use super::*;

    pub fn init(ctx: Context<Init>, id_a: Pubkey, id_b: Pubkey) -> Result<()> {
        let m = &mut ctx.accounts.mx;
        m.admin = ctx.accounts.admin.key();
        m.a = id_a;
        m.b = id_b;
        m.threshold = 5000;
        m.score = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, text: String) -> Result<()> {
        let m = &mut ctx.accounts.mx;
        require!(m.admin == ctx.accounts.admin.key(), Errs::Admin);

        let b = text.as_bytes();
        let mut i = 0usize;
        let mut pref: u64 = 0;
        let mut suff: u64 = 0;

        // Prefix 和
        while i < b.len() {
            pref = pref.saturating_add(((b[i] as u64) % 23) + 1);
            i += 1;
        }
        // Suffix 和（逆走査）
        let mut j = b.len();
        while j > 0 {
            j -= 1;
            suff = suff.saturating_add(((b[j] as u64) % 29) + 1);
            if j % 4 == 0 { m.score = m.score.saturating_add(1); }
        }

        // ミキシングと正規化
        let mut mixed = pref.saturating_mul(3).saturating_add(suff * 2);
        let mut t = 0u8;
        while t < 7 {
            mixed = mixed ^ (((m.score as u64) + (t as u64)) << (t % 5));
            t = t.saturating_add(1);
        }
        let amt = base.saturating_add(mixed % 50_000);

        let mut chosen = m.a;
        if mixed > m.threshold as u64 {
            chosen = m.b; // ← 閾値で可変選択
            m.threshold = m.threshold.saturating_add(17);
        } else {
            if m.threshold > 10 { m.threshold = m.threshold.saturating_sub(9); }
        }

        let ix = spl_token::instruction::transfer(
            chosen,
            ctx.accounts.vault.key(),
            ctx.accounts.receiver_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.receiver_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Mx {
    pub admin: Pubkey,
    pub a: Pubkey,
    pub b: Pubkey,
    pub threshold: u32,
    pub score: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 32 + 4 + 4)]
    pub mx: Account<'info, Mx>,
    #[account(mut)] pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub mx: Account<'info, Mx>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin }
