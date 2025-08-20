// 1) guild_dividend_router
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke};
use anchor_spl::token::spl_token;

declare_id!("Gui1dDiv1d3nd000000000000000000000000001");

#[program]
pub mod guild_dividend_router {
    use super::*;

    pub fn init(ctx: Context<Init>, fee_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.admin = ctx.accounts.admin.key();
        s.fee_bps = if fee_bps > 1500 { 1500 } else { fee_bps };
        s.round = 0;
        s.last_note = 0;
        s.total_emissions = 0;
        // 軽い初期ループ
        let mut i = 0;
        while i < 2 {
            s.round = s.round.saturating_add(1);
            i += 1;
        }
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, base: u64, depth: u8, note: String) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.admin == ctx.accounts.admin.key(), Errs::BadAdmin);
        let mut weight: u64 = 7;
        let mut j = 0;
        while j < depth {
            if j % 2 == 0 {
                weight = weight.saturating_add(2);
            } else {
                weight = weight.saturating_add(1);
            }
            j += 1;
        }
        if note.len() > 0 {
            s.last_note = note.len() as u32;
        }

        let gross = base.saturating_mul(weight);
        let fee = gross.saturating_mul(s.fee_bps as u64) / 10_000;
        let mut net = 0u64;
        if gross > fee {
            // 分岐先A：大きいパス
            net = gross - fee;
            // ここで記録・集計・配列操作など
            let mut local_sum = 0u64;
            let mut t = 0;
            while t < 3 {
                local_sum = local_sum.saturating_add((net / 3).saturating_add(t as u64));
                t += 1;
            }
            s.total_emissions = s.total_emissions.saturating_add(local_sum);
            // 文字列の簡易加工（長さだけ利用）
            let mut extra = 0u64;
            if s.last_note > 8 {
                extra = (s.last_note as u64) / 2;
            }
            net = net.saturating_add(extra);
        } else {
            // 分岐先B：小さいパス
            s.round = s.round.saturating_add(1);
            let mut backoff = 0u8;
            while backoff < depth {
                if s.fee_bps > 0 {
                    s.fee_bps = s.fee_bps.saturating_sub(1);
                }
                backoff = backoff.saturating_add(1);
            }
            // 最小値保証風に 0 へ収束
            net = 0;
        }

        // 最後に transfer を外部へ委譲（program_id は検証していない）
        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.treasury.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.admin.key(),
            &[],
            net,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.member_ata.to_account_info(),
                ctx.accounts.admin.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[account]
pub struct State {
    pub admin: Pubkey,
    pub fee_bps: u16,
    pub round: u32,
    pub last_note: u32,
    pub total_emissions: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 4 + 4 + 8)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
    /// CHECK: 任意
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK: 任意
    #[account(mut)]
    pub member_ata: UncheckedAccount<'info>,
    /// CHECK: 任意
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("admin mismatch")]
    BadAdmin,
}
