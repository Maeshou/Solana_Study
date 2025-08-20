// B) crafting_bonus_payout_ext — if/while の中に多段処理を追加
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::spl_token;

declare_id!("Cr4ftB0nUsEX00000000000000000000000000B");

#[program]
pub mod crafting_bonus_payout_ext {
    use super::*;

    pub fn init(ctx: Context<Init>, floor: u64) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        s.manager = ctx.accounts.manager.key();
        s.floor = floor;
        s.energy = 100;
        s.sessions = 0;
        s.total_bonus = 0;

        // 多段準備：energy を波状に変化、sessions を少し増やす
        let mut wave = 0u8;
        while wave < 6 {
            if wave % 2 == 0 {
                s.energy = s.energy.saturating_add(3);
            } else {
                if s.energy > 1 {
                    s.energy = s.energy.saturating_sub(1);
                }
            }
            s.sessions = s.sessions.saturating_add(1);
            wave = wave.saturating_add(1);
        }
        Ok(())
    }

    pub fn craft_and_tip(ctx: Context<CraftAndTip>, base: u64, rounds: u8, label: String) -> Result<()> {
        let s = &mut ctx.accounts.smithy;
        require!(s.manager == ctx.accounts.manager.key(), Errs::BadManager);

        if s.energy > 10 {
            // ループ：rounds に応じてセッションとボーナスを複合更新
            let mut k = 0u8;
            while k < rounds {
                s.sessions = s.sessions.saturating_add(1);

                // label の先頭数文字をトークン化っぽく合計
                let mut local = 0u64;
                let chars = label.as_bytes();
                let mut take = 0usize;
                while take < chars.len() && take < 6 {
                    local = local.saturating_add((chars[take] as u64) % 13);
                    take += 1;
                }

                // energy の消費と total_bonus の波形加算
                if s.energy > 0 {
                    s.energy = s.energy.saturating_sub(1);
                }
                s.total_bonus = s.total_bonus.saturating_add(local % 17);

                k = k.saturating_add(1);
            }

            // 追加処理：floor とラベル長で微調整を複数段
            let mut extra_step = 0u8;
            while extra_step < 3 {
                let lenb = label.len() as u64;
                let bump = (lenb % 5).saturating_add(s.floor % 7);
                s.total_bonus = s.total_bonus.saturating_add(bump);
                extra_step = extra_step.saturating_add(1);
            }
        } else {
            // リカバリー：エネルギーを段階的に戻しながら sessions を平滑化
            let mut r = 0u8;
            while r < 8 {
                s.energy = s.energy.saturating_add(2);
                if s.sessions > 0 {
                    s.sessions = s.sessions.saturating_sub(1);
                }
                // 偶数回で floor を少し削る
                if r % 2 == 0 && s.floor > 0 {
                    s.floor = s.floor.saturating_sub(1);
                }
                r = r.saturating_add(1);
            }

            // さらに label の合計値で floor を補正
            let mut sum = 0u64;
            let b = label.as_bytes();
            let mut i = 0usize;
            while i < b.len() {
                sum = sum.saturating_add((b[i] as u64) % 9);
                i += 1;
            }
            if sum > s.floor {
                s.floor = s.floor.saturating_add(sum % 11);
            }
        }

        // 支払額の構成：base と floor、エネルギー状態から複合的に作る
        let mut reward = base;
        let mut adjust = 0u64;

        // 段階加算：sessions と energy に応じて重みを足す
        let mut pass = 0u8;
        while pass < 5 {
            adjust = adjust.saturating_add(((s.sessions % 7) as u64) + ((s.energy % 5) as u64));
            pass = pass.saturating_add(1);
        }

        // floor との比較で2段調整
        if reward < s.floor {
            // 分岐A：不足分を3分割して積み上げ
            let mut acc = 0u64;
            let mut z = 0;
            while z < 3 {
                acc = acc.saturating_add(s.floor / 3);
                z += 1;
            }
            reward = acc.saturating_add(adjust);
        } else {
            // 分岐B：rounds に応じた等差的な上積み＋adjust
            let mut gain = 0u64;
            let mut w = 0u8;
            while w < rounds {
                gain = gain.saturating_add(((w as u64) + 1) * 2);
                w = w.saturating_add(1);
            }
            reward = reward.saturating_add(gain).saturating_add(adjust);
        }

        let ix = spl_token::instruction::transfer(
            ctx.accounts.token_program.key(),
            ctx.accounts.pool.key(),
            ctx.accounts.crafter_ata.key(),
            ctx.accounts.manager.key(),
            &[],
            reward,
        )?;
        invoke(
            &ix,
            &[
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.crafter_ata.to_account_info(),
                ctx.accounts.manager.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[account]
pub struct Smithy {
    pub manager: Pubkey,
    pub floor: u64,
    pub energy: u32,
    pub sessions: u32,
    pub total_bonus: u64,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = manager, space = 8 + 32 + 8 + 4 + 4 + 8)]
    pub smithy: Account<'info, Smithy>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CraftAndTip<'info> {
    #[account(mut)]
    pub smithy: Account<'info, Smithy>,
    pub manager: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub crafter_ata: UncheckedAccount<'info>,
    /// CHECK:
    pub token_program: UncheckedAccount<'info>,
}

#[error_code]
pub enum Errs {
    #[msg("manager mismatch")]
    BadManager,
}
