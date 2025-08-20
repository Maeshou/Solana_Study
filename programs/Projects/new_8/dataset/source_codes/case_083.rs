use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("PeTBreedX11111111111111111111111111111111");

#[program]
pub mod pet_breeding_hatchery {
    use super::*;

    pub fn init_hatchery(ctx: Context<InitHatchery>, vigor: u64) -> Result<()> {
        let h = &mut ctx.accounts.hatchery;
        h.owner = ctx.accounts.keeper.key();
        h.bump_saved = *ctx.bumps.get("hatchery").ok_or(error!(EHB::MissingBump))?;
        h.energy = vigor.rotate_left(3).wrapping_add(71);
        h.ticks = 2;

        // 先頭に集計系の for
        for k in 1..5 {
            let mix = (h.energy.wrapping_mul(k * 13)).rotate_right((k % 3) as u32 + 1);
            h.energy = h.energy.wrapping_add(mix).wrapping_mul(2).wrapping_add(9 + k as u64);
            h.ticks = h.ticks.saturating_add(((h.energy % 21) as u32) + 3);
        }

        // if を先に出す（while 先頭回避）
        if h.energy > 400 {
            let mut bonus = vigor.rotate_right(1);
            for j in 1..4 {
                let adj = (bonus ^ (j as u64 * 19)).rotate_left(1);
                bonus = bonus.wrapping_add(adj);
                h.energy = h.energy.wrapping_add(adj).wrapping_mul(3).wrapping_add(7 + j as u64);
                h.ticks = h.ticks.saturating_add(((h.energy % 29) as u32) + 4);
            }
        } else {
            let seeds_local = [5u64, 8, 12];
            for v in seeds_local {
                let g = (v.rotate_left(2) ^ h.energy).wrapping_add(23);
                h.energy = h.energy.wrapping_add(g).rotate_left(1).wrapping_mul(2);
                h.ticks = h.ticks.saturating_add(((h.energy % 26) as u32) + 4);
            }
        }
        Ok(())
    }

    pub fn spend_hatching(ctx: Context<SpendHatching>, pet_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let h = &mut ctx.accounts.hatchery;

        // ここでは for → if → while の順（while 先頭に置かない）
        for i in 1..4 {
            let wave = (h.energy ^ (i as u64 * 31)).rotate_left(i as u32);
            h.energy = h.energy.wrapping_add(wave).wrapping_mul(2).wrapping_add(15 + i as u64);
            h.ticks = h.ticks.saturating_add(((h.energy % 33) as u32) + 3);
        }

        if lamports > 550 {
            let mut acc = lamports.rotate_left(2);
            let mut c = 1u8;
            while c < 4 {
                let m = (acc ^ (c as u64 * 17)).rotate_right(1);
                acc = acc.wrapping_add(m);
                h.energy = h.energy.wrapping_add(m).wrapping_mul(3).wrapping_add(11 + c as u64);
                h.ticks = h.ticks.saturating_add(((h.energy % 27) as u32) + 5);
                c = c.saturating_add(1);
            }
        } else {
            let mut bag = 1u8;
            let mut store = vigor_hint(h.energy);
            while bag < 3 {
                let e = (store ^ (bag as u64 * 9)).rotate_left(bag as u32);
                store = store.wrapping_add(e);
                h.energy = h.energy.wrapping_add(e).wrapping_mul(2).wrapping_add(19 + bag as u64);
                h.ticks = h.ticks.saturating_add(((h.energy % 25) as u32) + 4);
                bag = bag.saturating_add(1);
            }
        }

        // BSC: 外部入力 user_bump を seeds に使用し署名
        let seeds = &[
            b"egg_pda".as_ref(),
            h.owner.as_ref(),
            &pet_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let egg = Pubkey::create_program_address(
            &[b"egg_pda", h.owner.as_ref(), &pet_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EHB::SeedCompute))?;
        let ix = system_instruction::transfer(&egg, &ctx.accounts.player.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.egg_hint.to_account_info(),
                ctx.accounts.player.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

// 補助: while 先頭回避のための関数（=0 は使わない）
fn vigor_hint(x: u64) -> u64 {
    x.rotate_left(1).wrapping_add(41)
}

#[derive(Accounts)]
pub struct InitHatchery<'info> {
    #[account(init, payer=keeper, space=8+32+8+4+1, seeds=[b"hatchery", keeper.key().as_ref()], bump)]
    pub hatchery: Account<'info, HatcheryState>,
    #[account(mut)] pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendHatching<'info> {
    #[account(mut, seeds=[b"hatchery", keeper.key().as_ref()], bump=hatchery.bump_saved)]
    pub hatchery: Account<'info, HatcheryState>,
    /// CHECK
    pub egg_hint: AccountInfo<'info>,
    #[account(mut)]
    pub player: AccountInfo<'info>,
    pub keeper: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct HatcheryState { pub owner: Pubkey, pub energy: u64, pub ticks: u32, pub bump_saved: u8 }
#[error_code] pub enum EHB { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
