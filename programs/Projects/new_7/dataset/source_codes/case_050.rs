// 4) runlength_aggregator — ランレングス圧縮風の集約、引数IDで実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("RunL3ngthAggr3g4tor444444444444444444444");

#[program]
pub mod runlength_aggregator {
    use super::*;

    pub fn setup(ctx: Context<Setup>, base_tick: u32) -> Result<()> {
        let a = &mut ctx.accounts.ag;
        a.owner = ctx.accounts.owner.key();
        a.tick = base_tick;
        a.total = 0;
        a.last = 0;
        a.burst = 0;

        // 交互に tick をアップダウン
        let mut i = 0u8;
        while i < 10 {
            if i % 2 == 0 { a.tick = a.tick.saturating_add(3); }
            else if a.tick > 0 { a.tick = a.tick.saturating_sub(1); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, program_id_arg: Pubkey, text: String, base: u64) -> Result<()> {
        let a = &mut ctx.accounts.ag;
        require!(a.owner == ctx.accounts.owner.key(), Errs::Owner);

        // ランレングス風
        let b = text.as_bytes();
        let mut i = 0usize;
        let mut run_val = 0u8;
        let mut run_len = 0u64;
        let mut acc = 0u64;

        while i < b.len() {
            if run_len == 0 {
                run_val = b[i];
                run_len = 1;
            } else {
                if b[i] == run_val {
                    run_len = run_len.saturating_add(1);
                } else {
                    acc = acc.saturating_add((run_len * ((run_val as u64) % 17 + 1)));
                    run_val = b[i];
                    run_len = 1;
                }
            }
            if i % 5 == 0 { a.burst = a.burst.saturating_add(1); }
            i += 1;
        }
        if run_len > 0 {
            acc = acc.saturating_add(run_len * ((run_val as u64) % 17 + 1));
        }

        // 後処理：last と total をステップ更新
        let mut k = 0u8;
        while k < 6 {
            a.last = a.last.saturating_add(((a.tick % 9) as u64) + (k as u64));
            a.total = a.total.saturating_add(a.last % 101);
            k = k.saturating_add(1);
        }

        let amt = base.saturating_add(acc).saturating_add(a.last % 1000);

        let ix = spl_token::instruction::transfer(
            program_id_arg, // ← 引数で渡されたID
            ctx.accounts.bank.key(),
            ctx.accounts.client_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.bank.to_account_info(),
            ctx.accounts.client_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Ag {
    pub owner: Pubkey,
    pub tick: u32,
    pub total: u64,
    pub last: u64,
    pub burst: u32,
}

#[derive(Accounts)]
pub struct Setup<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 4 + 8 + 8 + 4)]
    pub ag: Account<'info, Ag>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub ag: Account<'info, Ag>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub bank: Account<'info, TokenAccount>,
    #[account(mut)]
    pub client_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
