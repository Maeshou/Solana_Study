// 3) arena_rank_bonus
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Ar3naRankB0n0000000000000000000000000003");

#[program]
pub mod arena_rank_bonus {
    use super::*;

    pub fn open(ctx: Context<Open>, ladder: u8) -> Result<()> {
        let a = &mut ctx.accounts.arena;
        a.admin = ctx.accounts.admin.key();
        a.ladder = ladder;
        a.matches = 0;
        a.points = 0;
        a.cycle = 0;

        let mut j = 0u8;
        while j < 5 {
            a.points = a.points.saturating_add(((ladder as u32) % 7) + (j as u32));
            a.cycle = a.cycle.saturating_add(1);
            j = j.saturating_add(1);
        }
        Ok(())
    }

    pub fn record_and_pay(ctx: Context<RecordAndPay>, rank: u8, name: String) -> Result<()> {
        let a = &mut ctx.accounts.arena;
        require!(a.admin == ctx.accounts.admin.key(), Errs::Admin);

        if rank == 1 {
            // 首位：名前を走査しながらポイントに反映、ラダーで更に上積み
            let b = name.as_bytes();
            let mut i = 0usize;
            let mut local = 0u32;
            while i < b.len() {
                local = local.saturating_add((b[i] as u32) % 11 + 1);
                if i % 4 == 0 {
                    local = local.saturating_add(2);
                }
                i += 1;
            }
            a.points = a.points.saturating_add(local);
            let mut h = 0u8;
            while h < a.ladder {
                a.points = a.points.saturating_add((h % 3) as u32 + 1);
                h = h.saturating_add(1);
            }
        } else {
            // それ以外：サイクル進行とポイント調整を段階的に
            let mut step = 0u8;
            while step < 6 {
                a.cycle = a.cycle.saturating_add(1);
                if a.points > 0 && step % 2 == 0 {
                    a.points = a.points.saturating_sub(1);
                }
                step = step.saturating_add(1);
            }
            if name.len() < 5 {
                let mut pad = 0u8;
                while pad < 3 {
                    a.points = a.points.saturating_add(1);
                    pad = pad.saturating_add(1);
                }
            }
        }

        a.matches = a.matches.saturating_add(1);
        let mut pay = ((a.points as u64) / 3).saturating_add((a.cycle as u64) % 10);
        let mut bonus = 0u64;
        let mut c = 0u8;
        while c < 4 {
            bonus = bonus.saturating_add(((a.matches % 9) as u64) + (c as u64));
            c = c.saturating_add(1);
        }
        pay = pay.saturating_add(bonus);

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.pool.key(),
            ctx.accounts.winner_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            pay,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.winner_ata.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Arena {
    pub admin: Pubkey,
    pub ladder: u8,
    pub matches: u32,
    pub points: u32,
    pub cycle: u32,
}

#[derive(Accounts)]
pub struct Open<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 1 + 4 + 4 + 4)]
    pub arena: Account<'info, Arena>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct RecordAndPay<'info> {
    #[account(mut)]
    pub arena: Account<'info, Arena>,
    pub admin: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub winner_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("admin mismatch")] Admin }
