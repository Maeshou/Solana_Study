// 例2) Farm Registry + Plot Cell（手動: [b"plot_cell", farmer, plot_id] + user_bump）
use anchor_lang::prelude::*; use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
declare_id!("FaRmReGiStRy1111111111111111111111111111");

#[program]
pub mod farm_plot_cell {
    use super::*;
    pub fn register_farm(ctx: Context<RegisterFarm>, area: u64) -> Result<()> {
        let f = &mut ctx.accounts.farm;
        f.farmer = ctx.accounts.farmer.key();
        f.bump_main = *ctx.bumps.get("farm").ok_or(error!(EE::MissingBump))?;
        f.yield_hint = area % 3_000 + 700;
        f.ticks = 1;

        // for → if → while（各ブロック長め）
        for i in 0..4 {
            f.yield_hint = f.yield_hint.wrapping_add(23 + i as u64 * 17);
            f.ticks = f.ticks.saturating_add(((f.yield_hint % 37) as u32) + 5);
            f.yield_hint = f.yield_hint.rotate_right(1).wrapping_add(31);
            f.ticks = f.ticks.saturating_add(2);
        }
        if f.ticks & 1 == 1 {
            f.yield_hint = f.yield_hint.rotate_left(1).wrapping_add(29);
            f.ticks = f.ticks.saturating_add(11);
            f.yield_hint = f.yield_hint.wrapping_mul(2).wrapping_add(13);
            f.ticks = f.ticks.saturating_add(4);
        } else {
            f.yield_hint = f.yield_hint.rotate_right(2).wrapping_add(17);
            f.ticks = f.ticks.saturating_add(7);
            f.yield_hint = f.yield_hint.wrapping_add(59);
            f.ticks = f.ticks.saturating_add(3);
        }
        let mut w = 0u8;
        while w < 3 {
            f.yield_hint = f.yield_hint.wrapping_add(19 + w as u64 * 11);
            f.ticks = f.ticks.saturating_add(((f.yield_hint % 19) as u32) + 6);
            f.yield_hint = f.yield_hint.rotate_left(2).wrapping_add(21);
            f.ticks = f.ticks.saturating_add(2);
            w = w.saturating_add(1);
        }
        Ok(())
    }

    pub fn draw_from_plot(ctx: Context<DrawFromPlot>, plot_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let f = &mut ctx.accounts.farm;

        // while → for → if
        let mut k = 0u8;
        while k < 2 {
            f.yield_hint = f.yield_hint.wrapping_add((plot_id % 41) + 7 + k as u64);
            f.ticks = f.ticks.saturating_add(((lamports % 29) as u32) + 4 + k as u32);
            f.yield_hint = f.yield_hint.rotate_right(1).wrapping_add(15);
            f.ticks = f.ticks.saturating_add(3);
            k = k.saturating_add(1);
        }
        for j in 0..3 {
            f.yield_hint = f.yield_hint.wrapping_mul(2).wrapping_add(9 + j as u64);
            f.ticks = f.ticks.saturating_add(((f.yield_hint % 17) as u32) + 5);
            f.yield_hint = f.yield_hint.rotate_left(2).wrapping_add(27);
            f.ticks = f.ticks.saturating_add(2);
        }
        if f.ticks & 2 == 2 {
            f.yield_hint = f.yield_hint.rotate_right(2).wrapping_add(33);
            f.ticks = f.ticks.saturating_add(10);
            f.yield_hint = f.yield_hint.wrapping_add(44);
            f.ticks = f.ticks.saturating_add(4);
        } else {
            f.yield_hint = f.yield_hint.rotate_left(1).wrapping_add(25);
            f.ticks = f.ticks.saturating_add(8);
            f.yield_hint = f.yield_hint.wrapping_add(36);
            f.ticks = f.ticks.saturating_add(3);
        }

        let seeds = &[
            b"plot_cell".as_ref(),
            f.farmer.as_ref(),
            &plot_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"plot_cell", f.farmer.as_ref(), &plot_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.collector.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.plot_cell_hint.to_account_info(),
                ctx.accounts.collector.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct RegisterFarm<'info>{
    #[account(init,payer=farmer,space=8+32+8+4+1,seeds=[b"farm",farmer.key().as_ref()],bump)]
    pub farm: Account<'info,FarmState>, #[account(mut)] pub farmer: Signer<'info>, pub system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct DrawFromPlot<'info>{
    #[account(mut,seeds=[b"farm",farmer.key().as_ref()],bump=farm.bump_main)]
    pub farm: Account<'info,FarmState>, /// CHECK
    pub plot_cell_hint: AccountInfo<'info>, /// CHECK
    #[account(mut)] pub collector: AccountInfo<'info>, pub farmer: Signer<'info>, pub system_program: Program<'info,System>,
}
#[account] pub struct FarmState{ pub farmer: Pubkey, pub yield_hint: u64, pub ticks: u32, pub bump_main: u8 }
#[error_code] pub enum EE{ #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
