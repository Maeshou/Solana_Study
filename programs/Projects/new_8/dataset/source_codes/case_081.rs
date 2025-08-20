use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("CrAfTStAtIoNX333333333333333333333333333");

#[program]
pub mod craft_station_pipeline {
    use super::*;

    pub fn build_station(ctx: Context<BuildStation>, hint: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.owner = ctx.accounts.maker.key();
        s.bump_main = *ctx.bumps.get("station").ok_or(error!(EE::MissingBump))?;
        s.charge = hint.rotate_left(1).wrapping_add(605);
        s.turn = 3;

        // パイプライン：素材→溶融→鍛造→研磨の段
        let mats = [2u64, 3, 5, 8, 13, 21];
        for (i, m) in mats.iter().enumerate() {
            let a = m.rotate_left(((i as u32) % 3) + 1);
            s.charge = s.charge.wrapping_add(a).wrapping_mul(2);
            s.turn = s.turn.saturating_add(((s.charge % 24) as u32) + 3);

            let mut heat = 1u8;
            while heat < 4 {
                let melt = (s.charge ^ (*m + heat as u64)).rotate_right(1);
                s.charge = s.charge.wrapping_add(melt).wrapping_mul(3).wrapping_add(17 + heat as u64);
                s.turn = s.turn.saturating_add(((s.charge % 17) as u32) + 2);
                heat = heat.saturating_add(1);
            }
        }

        // 分岐：charge閾値で仕上げが変わる
        if s.charge > 820 {
            let mut p = 1u8;
            let mut carry = 0u64;
            while p < 5 {
                let polish = (s.charge ^ (p as u64 * 19)).rotate_left(p as u32);
                carry = carry.wrapping_add(polish);
                s.charge = s.charge.wrapping_add(polish).wrapping_mul(2);
                s.turn = s.turn.saturating_add(((s.charge % 29) as u32) + 4);
                p = p.saturating_add(1);
            }
            s.charge = s.charge.rotate_right(1).wrapping_add(carry);
            s.turn = s.turn.saturating_add(((carry % 23) as u32) + 3);
        } else {
            let coarse = [11u64, 14, 15];
            for c in coarse {
                let grind = (c.rotate_left(2) ^ s.charge).wrapping_add(31);
                s.charge = s.charge.wrapping_add(grind).wrapping_mul(2).rotate_left(1);
                s.turn = s.turn.saturating_add(((s.charge % 26) as u32) + 4);
            }
        }
        Ok(())
    }

    pub fn spend_queue(ctx: Context<SpendQueue>, queue_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;

        // キューの段階的消費
        for n in 0..5 {
            let q = (queue_id.rotate_left(n) ^ s.charge).wrapping_add(7 + n as u64);
            s.charge = s.charge.wrapping_add(q).rotate_right(1).wrapping_mul(2);
            s.turn = s.turn.saturating_add(((s.charge % 33) as u32) + 3);
        }

        if lamports > 560 {
            let mut acc = lamports.rotate_left(2);
            for j in 0..3 {
                let d = (acc ^ (j as u64 * 37)).rotate_left(1);
                s.charge = s.charge.wrapping_add(d).wrapping_mul(3).wrapping_add(23 + j as u64);
                s.turn = s.turn.saturating_add(((s.charge % 28) as u32) + 5);
                acc = acc.rotate_right(1).wrapping_add(d);
            }
            s.charge = s.charge.rotate_left(2).wrapping_add(acc);
            s.turn = s.turn.saturating_add(((acc % 17) as u32) + 4);
        } else {
            let mut r = 1u8;
            let mut bag = 0u64;
            while r < 4 {
                let e = (s.charge ^ (r as u64 * 9)).rotate_right(1);
                bag = bag.wrapping_add(e);
                s.charge = s.charge.wrapping_add(e).wrapping_mul(2).wrapping_add(29 + r as u64);
                s.turn = s.turn.saturating_add(((s.charge % 21) as u32) + 5);
                r = r.saturating_add(1);
            }
            s.charge = s.charge.rotate_left(1).wrapping_add(bag);
            s.turn = s.turn.saturating_add(((bag % 19) as u32) + 3);
        }

        // 未検証 queue_cell へ署名
        let seeds = &[
            b"queue_cell".as_ref(),
            s.owner.as_ref(),
            &queue_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"queue_cell", s.owner.as_ref(), &queue_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.consumer.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.queue_cell_hint.to_account_info(),
                ctx.accounts.consumer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuildStation<'info> {
    #[account(init, payer=maker, space=8+32+8+4+1, seeds=[b"station", maker.key().as_ref()], bump)]
    pub station: Account<'info, StationState>,
    #[account(mut)] pub maker: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendQueue<'info> {
    #[account(mut, seeds=[b"station", maker.key().as_ref()], bump=station.bump_main)]
    pub station: Account<'info, StationState>,
    /// CHECK 未検証
    pub queue_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub consumer: AccountInfo<'info>,
    pub maker: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct StationState { pub owner: Pubkey, pub charge: u64, pub turn: u32, pub bump_main: u8 }
#[error_code] pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
