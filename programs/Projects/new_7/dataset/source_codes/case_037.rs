// 1) rotating_program_selector — 状態に配列で保持したIDを周回選択
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("R0tatingPr0gSel1111111111111111111111111");

#[program]
pub mod rotating_program_selector {
    use super::*;

    pub fn init(ctx: Context<Init>, ids: Vec<Pubkey>) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.cursor = 0;
        s.ids = ids;
        s.pulse = 0;

        let mut i = 0u8;
        while i < 6 {
            s.pulse = s.pulse.saturating_add((i as u32) + 1);
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, note: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::Admin);
        require!(s.ids.len() > 0, Errs::Empty);

        // カーソルを進めつつ選択
        let mut idx = s.cursor as usize;
        if idx >= s.ids.len() { idx = 0; }
        let chosen = s.ids[idx];

        s.cursor = s.cursor.saturating_add(1);
        if (s.cursor as usize) >= s.ids.len() { s.cursor = 0; }

        // ノート処理で重み
        let mut w: u64 = 7;
        let b = note.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            w = w.saturating_add((b[i] as u64) % 9 + 1);
            if i % 3 == 0 && s.pulse > 0 {
                s.pulse = s.pulse.saturating_sub(1);
            }
            i += 1;
        }

        let mut amt = base.saturating_mul(w);
        let mut boost = 0u64;
        let mut k = 0u8;
        while k < 4 {
            boost = boost.saturating_add((k as u64) + ((s.pulse % 7) as u64));
            k = k.saturating_add(1);
        }
        amt = amt.saturating_add(boost);

        let ix = spl_token::instruction::transfer(
            chosen, // ← 状態配列から選ばれた program_id
            ctx.accounts.treasury.key(),
            ctx.accounts.recipient_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.recipient_ata.to_account_info(),
            ctx.accounts.admin.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub cursor: u32,
    pub ids: Vec<Pubkey>,
    pub pulse: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 4 + (32*8) + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("admin mismatch")] Admin, #[msg("ids empty")] Empty }
