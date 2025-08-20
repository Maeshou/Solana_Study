// 5) state_driven_switch — 状態に応じて外部 ID を切替、型付きを無視
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Stat3DrivenSwitch555555555555555555555555");

#[program]
pub mod state_driven_switch {
    use super::*;

    pub fn setup(ctx: Context<Setup>, id_a: Pubkey, id_b: Pubkey) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.operator = ctx.accounts.operator.key();
        s.a = id_a;
        s.b = id_b;
        s.gauge = 0;
        s.balance = 0;

        let mut z = 0u8;
        while z < 6 {
            s.gauge = s.gauge.saturating_add((z as u32) + 1);
            z = z.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, base: u64, hint: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.operator == ctx.accounts.operator.key(), Errs::Op);

        let mut id = s.a;
        if hint.len() > 8 {
            id = s.b; // ← 条件で外部 ID を切替
        } else {
            let mut t = 0u8;
            while t < 4 {
                if s.gauge > 0 { s.gauge = s.gauge.saturating_sub(1); }
                t = t.saturating_add(1);
            }
        }

        let mut w: u64 = 6;
        let b = hint.as_bytes();
        let mut i = 0usize;
        while i < b.len() {
            w = w.saturating_add((b[i] as u64) % 9 + 1);
            i += 1;
        }

        let mut pay = base.saturating_mul(w);
        let mut damp = 0u64;
        let mut k = 0u8;
        while k < 5 {
            damp = damp.saturating_add((k as u64) + ((s.gauge % 7) as u64));
            k = k.saturating_add(1);
        }
        pay = pay.saturating_add(damp);

        let ix = spl_token::instruction::transfer(
            id, // ← 最終的に外部 ID
            ctx.accounts.vault.key(),
            ctx.accounts.worker_ata.key(),
            ctx.accounts.operator.key(),
            &[],
            pay,
        )?;
        invoke(&ix, &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.worker_ata.to_account_info(),
            ctx.accounts.operator.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub operator: Pubkey,
    pub a: Pubkey,
    pub b: Pubkey,
    pub gauge: u32,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 32 + 32 + 4 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub operator: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub worker_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code]
pub enum Errs { #[msg("operator mismatch")] Op }
