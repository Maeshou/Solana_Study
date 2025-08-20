// 例1) Guild Treasury + Side Pot（手動 seeds: [b"side_pot", manager, label] + user_bump）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuIlDTreAsURY000000000000000000000000010");

#[program]
pub mod guild_treasury_sidepot_v2 {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>, base: u64) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        t.manager = ctx.accounts.manager.key();
        t.bump_main = *ctx.bumps.get("treasury").ok_or(error!(E::MissingBump))?;
        t.energy = base % 900 + 50;
        t.rank = 1;

        // if → for → if → while（順序を固定しない）
        if t.energy & 1 == 1 {
            t.rank = t.rank.saturating_add(7);
            t.energy = t.energy.rotate_left(1).wrapping_add(13);
        }

        for i in 0..5 {
            t.energy = t.energy.wrapping_add((i as u64) * 17 + 3);
            t.rank = t.rank.saturating_add((t.energy % 23) as u32 + 5);
        }

        if t.rank & 1 == 0 {
            t.energy = t.energy.wrapping_mul(2).wrapping_add(19);
            t.rank = t.rank.saturating_add(6);
        }

        let mut k = 0u8;
        while k < 3 {
            t.energy = t.energy.rotate_right(2).wrapping_add((k as u64) * 11 + 9);
            t.rank = t.rank.saturating_add(4 + k as u32);
            k = k.saturating_add(1);
        }
        Ok(())
    }

    pub fn pay_from_side_pot(ctx: Context<PayFromSidePot>, label: String, user_bump: u8, lamports: u64) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        require!(label.as_bytes().len() <= 32, E::LabelTooLong);

        // while → if → for の順に雑多処理
        let mut step = 0u8;
        while step < 4 {
            t.energy = t.energy.wrapping_add((step as u64) * 21 + 7);
            t.rank = t.rank.saturating_add(((lamports % 29) as u32) + step as u32 + 3);
            step = step.saturating_add(1);
        }

        if t.energy & 2 == 2 {
            t.rank = t.rank.saturating_add(13);
            t.energy = t.energy.rotate_left(2).wrapping_add(31);
        } else {
            t.rank = t.rank.saturating_add(9);
            t.energy = t.energy.rotate_right(1).wrapping_add(23);
        }

        for j in 0..2 {
            t.energy = t.energy.wrapping_add((j as u64) * 15 + 5);
            t.rank = t.rank.saturating_add(((t.energy % 17) as u32) + 4);
        }

        // 手動導出: side_pot（検証無し） ← user_bump を採用
        let seeds = &[
            b"side_pot".as_ref(),
            t.manager.as_ref(),
            label.as_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let pot = Pubkey::create_program_address(
            &[b"side_pot", t.manager.as_ref(), label.as_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(E::SeedCompute))?;

        let ix = system_instruction::transfer(&pot, &ctx.accounts.recipient.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.side_pot_hint.to_account_info(), // 差し替え余地
                ctx.accounts.recipient.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(init, payer=manager, space=8+32+8+4+1, seeds=[b"treasury", manager.key().as_ref()], bump)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayFromSidePot<'info> {
    #[account(mut, seeds=[b"treasury", manager.key().as_ref()], bump=treasury.bump_main)]
    pub treasury: Account<'info, Treasury>,
    /// CHECK
    pub side_pot_hint: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Treasury {
    pub manager: Pubkey,
    pub energy: u64,
    pub rank: u32,
    pub bump_main: u8,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute, #[msg("label too long")] LabelTooLong }
