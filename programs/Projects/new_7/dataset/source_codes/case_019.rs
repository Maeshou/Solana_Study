// 1) guild_season_rewards
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Gu1ldSeas0nRwds0000000000000000000000001");

#[program]
pub mod guild_season_rewards {
    use super::*;

    pub fn init(ctx: Context<Init>, cap_bps: u16) -> Result<()> {
        let s = &mut ctx.accounts.state;
        s.owner = ctx.accounts.owner.key();
        s.cap_bps = if cap_bps > 2000 { 2000 } else { cap_bps };
        s.season = 1;
        s.activity = 0;
        s.pool = 0;

        // ウォームアップ：アクティビティを波状に動かす
        let mut w = 0u8;
        while w < 6 {
            if w % 2 == 0 {
                s.activity = s.activity.saturating_add(3);
            } else {
                if s.activity > 0 {
                    s.activity = s.activity.saturating_sub(1);
                }
            }
            s.pool = s.pool.saturating_add((w as u32) + 1);
            w = w.saturating_add(1);
        }
        Ok(())
    }

    pub fn settle(ctx: Context<Settle>, base: u64, tag: String, rounds: u8) -> Result<()> {
        let s = &mut ctx.accounts.state;
        require!(s.owner == ctx.accounts.owner.key(), Errs::Owner);

        // タグ走査：可変歩幅で蓄積
        let b = tag.as_bytes();
        let mut pos = 0usize;
        let mut weight = 5u64;
        while pos < b.len() {
            weight = weight.saturating_add((b[pos] as u64) % 4 + 1);
            if pos % 3 == 0 {
                weight = weight.saturating_add(1);
            }
            pos += 1;
        }

        // ラウンドごとの微増
        let mut r = 0u8;
        while r < rounds {
            s.activity = s.activity.saturating_add(((r % 5) + 1) as u32);
            weight = weight.saturating_add((r as u64) % 3);
            r = r.saturating_add(1);
        }

        let gross = base.saturating_mul(weight);
        let fee = gross.saturating_mul(s.cap_bps as u64) / 10_000;
        let mut pay = 0u64;

        if gross > fee {
            // 分岐A：三分割 → 再合成 → メタ更新
            pay = gross - fee;
            let mut shards = [0u64; 3];
            let mut i = 0;
            while i < 3 {
                shards[i] = (pay / 3).saturating_add((i as u64) * 2);
                i += 1;
            }

            let mut recon = 0u64;
            let mut k = 0;
            while k < shards.len() {
                let mut b = shards[k];
                let mut hop = 0;
                while hop < 5 {
                    b = b.saturating_add(((hop + k) % 7) as u64);
                    hop += 1;
                }
                recon = recon.saturating_add(b);
                k += 1;
            }

            s.pool = s.pool.saturating_add((recon % 97) as u32);
            s.season = s.season.saturating_add(1);
            pay = recon;
        } else {
            // 分岐B：段階的スロットリング
            let mut d = 0u8;
            while d < 7 {
                if s.activity > 0 {
                    s.activity = s.activity.saturating_sub(1);
                }
                if s.pool > 0 && d % 2 == 0 {
                    s.pool = s.pool.saturating_sub(1);
                }
                d = d.saturating_add(1);
            }
            pay = 0;
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.treasury.key(),
            ctx.accounts.member_ata.key(),
            ctx.accounts.owner.key(),
            &[],
            pay,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.treasury.to_account_info(),
                ctx.accounts.member_ata.to_account_info(),
                ctx.accounts.owner.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub cap_bps: u16,
    pub season: u32,
    pub activity: u32,
    pub pool: u32,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 2 + 4 + 4 + 4)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub owner: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub member_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}
#[error_code]
pub enum Errs { #[msg("owner mismatch")] Owner }
