use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuIlDTreAsuRyX1111111111111111111111111");

#[program]
pub mod guild_treasury_stage {
    use super::*;

    pub fn init_hall(ctx: Context<InitHall>, seed: u64) -> Result<()> {
        let h = &mut ctx.accounts.hall;
        h.owner = ctx.accounts.master.key();
        h.bump_main = *ctx.bumps.get("hall").ok_or(error!(EE::MissingBump))?;
        h.power = seed.rotate_left(2).wrapping_add(733);
        h.steps = 3;

        // 前処理：多段回転＋素片加算（小ループを複合）
        let primes = [3u64, 5, 11, 17, 19, 23];
        for (i, p) in primes.iter().enumerate() {
            let spin = ((i as u32) % 4) + 1;
            h.power = h.power.rotate_left(spin).wrapping_add(p.wrapping_mul(7));
            h.steps = h.steps.saturating_add(((h.power % 21) as u32) + 4);

            let mut inner = 1u8;
            while inner < 3 {
                let blend = (h.power ^ (*p + inner as u64)).rotate_right(inner as u32);
                h.power = h.power.wrapping_add(blend).wrapping_mul(2).wrapping_add(13);
                h.steps = h.steps.saturating_add(((h.power % 17) as u32) + 3);
                inner = inner.saturating_add(1);
            }
        }

        // 分岐①：powerの閾値で重作業（各分岐の中身は複数行＋ネスト）
        if h.power > 900 {
            let mut acc = 0u64;
            for k in 0..4 {
                let fold = ((h.power ^ (k as u64 * 29)).rotate_left(1)).wrapping_add(37 + k as u64);
                acc = acc.wrapping_add(fold);
                h.power = h.power.rotate_right(((h.steps % 5) + 1) as u32).wrapping_add(fold);
                h.steps = h.steps.saturating_add(((h.power % 33) as u32) + 5);
            }
            h.power = h.power.wrapping_add(acc).wrapping_mul(2).wrapping_add(101);
            h.steps = h.steps.saturating_add(((acc % 27) as u32) + 6);
        } else {
            let mut trail = h.power.rotate_left(2).wrapping_add(47);
            let mut i = 1u8;
            while i < 4 {
                let mix = (trail ^ (i as u64 * 13)).rotate_right(i as u32);
                h.power = h.power.wrapping_add(mix).wrapping_mul(3).wrapping_add(19 + i as u64);
                h.steps = h.steps.saturating_add(((h.power % 25) as u32) + 4);
                trail = trail.rotate_left(1).wrapping_add(mix);
                i = i.saturating_add(1);
            }
            h.power = h.power.rotate_left(1).wrapping_add(trail).wrapping_mul(2);
            h.steps = h.steps.saturating_add(((trail % 19) as u32) + 3);
        }

        Ok(())
    }

    pub fn payout_cell(ctx: Context<PayoutCell>, member_slot: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let h = &mut ctx.accounts.hall;

        // 準備：スロット系列でスキャン
        let window = [7u64, 12, 18, 27, 44];
        for (i, w) in window.iter().enumerate() {
            let delta = (member_slot ^ (w.rotate_left(((i as u32) % 3) + 1))).wrapping_add(5 + i as u64);
            h.power = h.power.wrapping_add(delta).wrapping_mul(2).rotate_right(1);
            h.steps = h.steps.saturating_add(((h.power % 37) as u32) + 3);
        }

        // 分岐②：lamportsと偶奇の二段判定（両方とも中身を厚く）
        if lamports > 650 {
            let mut carry = lamports.rotate_left(2);
            for r in 0..3 {
                let bump = (carry ^ (r as u64 * 31)).rotate_left(1);
                h.power = h.power.wrapping_add(bump).wrapping_mul(2).wrapping_add(23 + r as u64);
                h.steps = h.steps.saturating_add(((h.power % 29) as u32) + 4);
                carry = carry.rotate_right(1).wrapping_add(bump);
            }
            h.power = h.power.wrapping_add(carry).rotate_left(((h.steps % 4) + 1) as u32);
            h.steps = h.steps.saturating_add(((carry % 17) as u32) + 3);
        } else {
            let mut scan = 1u8;
            while scan < 4 {
                let inc = (member_slot + scan as u64 * 9).rotate_right(scan as u32);
                h.power = h.power.wrapping_add(inc).wrapping_mul(3).wrapping_add(37 + scan as u64);
                h.steps = h.steps.saturating_add(((h.power % 23) as u32) + 5);
                scan = scan.saturating_add(1);
            }
            h.power = h.power.rotate_left(2).wrapping_add(61);
            h.steps = h.steps.saturating_add(6);
        }
        if h.steps & 1 > 0 {
            // 偶奇ブランチも十分な処理量
            let mut acc = 0u64;
            for t in 0..3 {
                let m = ((h.power ^ (t as u64 * 7)).rotate_left(1)).wrapping_add(17 + t as u64);
                acc = acc.wrapping_add(m);
                h.power = h.power.rotate_right(1).wrapping_add(m).wrapping_mul(2);
                h.steps = h.steps.saturating_add(((h.power % 21) as u32) + 3);
            }
            h.power = h.power.wrapping_add(acc).rotate_left(1);
            h.steps = h.steps.saturating_add(2);
        } else {
            let mut chain = h.power;
            for t in 0..4 {
                chain = chain.rotate_left(((t % 3) + 1) as u32).wrapping_add(29 + t as u64);
                h.power = h.power.wrapping_add(chain).wrapping_mul(2);
                h.steps = h.steps.saturating_add(((h.power % 31) as u32) + 4);
            }
        }

        // 未検証の treasury_cell PDA に user_bump で署名（危険）
        let seeds = &[
            b"treasury_cell".as_ref(),
            h.owner.as_ref(),
            &member_slot.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"treasury_cell", h.owner.as_ref(), &member_slot.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.receiver.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.treasury_cell_hint.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitHall<'info> {
    #[account(init, payer=master, space=8+32+8+4+1, seeds=[b"hall", master.key().as_ref()], bump)]
    pub hall: Account<'info, HallState>,
    #[account(mut)] pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayoutCell<'info> {
    #[account(mut, seeds=[b"hall", master.key().as_ref()], bump=hall.bump_main)]
    pub hall: Account<'info, HallState>,
    /// CHECK 未検証
    pub treasury_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub master: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct HallState { pub owner: Pubkey, pub power: u64, pub steps: u32, pub bump_main: u8 }
#[error_code] pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
