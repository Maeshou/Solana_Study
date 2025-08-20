// 6) sliding_variance_pool — 分散近似（スライディング）で重み、引数IDで実行
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use anchor_spl::token::spl_token;
use anchor_lang::solana_program::program::invoke;

declare_id!("S1idingVar1anc3P066666666666666666666666");

#[program]
pub mod sliding_variance_pool {
    use super::*;

    pub fn configure(ctx: Context<Configure>, k: u8) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.owner = ctx.accounts.owner.key();
        p.window = k.max(2);
        p.sum = 0;
        p.sum2 = 0;
        p.n = 0;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, external_id: Pubkey, base: u64, memo: String) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        require!(p.owner == ctx.accounts.owner.key(), Errs::Owner);

        let b = memo.as_bytes();
        let mut i = 0usize;
        let mut q: u64 = 0;

        // スライディング分散近似
        let w = p.window as usize;
        while i < b.len() {
            let val = (b[i] % 31) as u64 + 1;
            p.sum = p.sum.saturating_add(val);
            p.sum2 = p.sum2.saturating_add(val * val);
            p.n = p.n.saturating_add(1);
            if p.n as usize > w {
                let back = (b[i - w] % 31) as u64 + 1;
                p.sum = p.sum.saturating_sub(back);
                p.sum2 = p.sum2.saturating_sub(back * back);
                p.n = (w as u32);
            }
            let mean2 = (p.sum * p.sum) / (p.n.max(1) as u64);
            let var_approx = p.sum2.saturating_sub(mean2);
            q = q.saturating_add(var_approx % 100);
            i += 1;
        }

        // 追加の正規化
        let mut t = 0u8;
        while t < 6 {
            q = q.saturating_add(((p.n % 11) as u64) + (t as u64));
            t = t.saturating_add(1);
        }
        let amt = base.saturating_add(q);

        let ix = spl_token::instruction::transfer(
            external_id, // ← 引数に依存
            ctx.accounts.treasury.key(),
            ctx.accounts.dest_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            amt,
        )?;
        invoke(&ix, &[
            ctx.accounts.treasury.to_account_info(),
            ctx.accounts.dest_ata.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ])?;
        Ok(())
    }
}

#[account]
pub struct Pool {
    pub owner: Pubkey,
    pub window: u8,
    pub sum: u64,
    pub sum2: u64,
    pub n: u32,
}

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1 + 8 + 8 + 4)]
    pub pool: Account<'info, Pool>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dest_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
#[error_code] pub enum Errs { #[msg("owner mismatch")] Owner }
