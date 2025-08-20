use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("CrAfTStAtIoN3333333333333333333333333333");

#[program]
pub mod craft_station_router {
    use super::*;

    pub fn init_station(ctx: Context<InitStation>, salt: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;
        s.owner = ctx.accounts.artisan.key();
        s.bump_main = *ctx.bumps.get("station").ok_or(error!(EE::MissingBump))?;
        s.charge = salt.rotate_left(1).wrapping_add(615);
        s.turn = 3;

        // 折り畳み風：定数列を重ねる
        let mats = [2u64, 3, 5, 7, 11, 13];
        for (i, m) in mats.iter().enumerate() {
            s.charge = s.charge.wrapping_add(m.rotate_left(((i as u32) % 3) + 1)).wrapping_mul(2);
            s.turn = s.turn.saturating_add(((s.charge % 24) as u32) + 3);
            let mut spin = 1u8;
            while spin < 3 {
                s.charge = s.charge.rotate_right(spin as u32).wrapping_add(23 + spin as u64);
                s.turn = s.turn.saturating_add(((s.charge % 17) as u32) + 2);
                spin = spin.saturating_add(1);
            }
        }
        if s.charge > 800 {
            s.charge = s.charge.rotate_left(2).wrapping_add(49);
            s.turn = s.turn.saturating_add(6);
        } else {
            s.charge = s.charge.rotate_right(1).wrapping_add(37);
            s.turn = s.turn.saturating_add(4);
        }
        Ok(())
    }

    pub fn spend_from_queue(ctx: Context<SpendFromQueue>, queue_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let s = &mut ctx.accounts.station;

        // 位相シフト+ビット制御
        for shift in 0..4u32 {
            let mix = (queue_id.rotate_left(shift) ^ s.charge).rotate_right(((s.turn % 3) + 1) as u32);
            s.charge = s.charge.wrapping_add(mix).wrapping_mul(2).wrapping_add(13 + shift as u64);
            s.turn = s.turn.saturating_add(((s.charge % 31) as u32) + 3);
        }
        if lamports > 520 {
            s.charge = s.charge.rotate_left(2).wrapping_add(33);
            s.turn = s.turn.saturating_add(7);
        } else {
            s.charge = s.charge.rotate_right(2).wrapping_add(21);
            s.turn = s.turn.saturating_add(5);
        }
        if s.turn & 1 == 1 {
            s.charge = s.charge.wrapping_mul(3).wrapping_add(27);
            s.turn = s.turn.saturating_add(3);
        } else {
            s.charge = s.charge.wrapping_add(19).rotate_left(1);
            s.turn = s.turn.saturating_add(2);
        }

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
pub struct InitStation<'info> {
    #[account(init, payer=artisan, space=8+32+8+4+1, seeds=[b"station", artisan.key().as_ref()], bump)]
    pub station: Account<'info, StationState>,
    #[account(mut)] pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendFromQueue<'info> {
    #[account(mut, seeds=[b"station", artisan.key().as_ref()], bump=station.bump_main)]
    pub station: Account<'info, StationState>,
    /// CHECK 未検証
    pub queue_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub consumer: AccountInfo<'info>,
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct StationState { pub owner: Pubkey, pub charge: u64, pub turn: u32, pub bump_main: u8 }
#[error_code] pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
