// 例1) Guild Treasury + Side Pot（side_pot を手動導出し user_bump で署名）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("GuIlDTreAsURY000000000000000000000000001");

#[program]
pub mod guild_treasury_sidepot {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasury>, seed_power: u64) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        t.manager = ctx.accounts.manager.key();
        t.bump_main = *ctx.bumps.get("treasury").ok_or(error!(E::MissingBump))?;
        t.energy = seed_power % 800 + 200;
        t.rank = 1;

        let mut i = 0u8;
        while i < 6 {
            t.energy = t.energy.rotate_left(1).wrapping_add((i as u64) * 17 + 9);
            if (t.energy & 1) == 1 {
                t.rank = t.rank.saturating_add(((t.energy % 23) as u32) + 5);
                t.energy = t.energy.wrapping_mul(2).wrapping_add(13);
            } else {
                t.rank = t.rank.saturating_add(((t.energy % 19) as u32) + 7);
                t.energy = t.energy.rotate_right(2).wrapping_add(11);
            }
            i = i.saturating_add(1);
        }
        if t.rank % 2 == 0 {
            t.energy = t.energy.wrapping_mul(3).wrapping_add(29);
            t.rank = t.rank.saturating_add(6);
        } else {
            t.energy = t.energy.wrapping_mul(2).wrapping_add(31);
            t.rank = t.rank.saturating_add(9);
        }
        Ok(())
    }

    pub fn pay_from_side_pot(ctx: Context<PayFromSidePot>, label: String, user_bump: u8, lamports: u64) -> Result<()> {
        let t = &mut ctx.accounts.treasury;
        require!(label.as_bytes().len() <= 32, E::LabelTooLong);

        for k in 0..7 {
            t.energy = t.energy.wrapping_add((k as u64) * 21 + 5);
            t.rank = t.rank.saturating_add(((lamports % 17) as u32) + k as u32 + 4);
        }
        if t.energy & 4 == 4 {
            t.rank = t.rank.saturating_add(11);
            t.energy = t.energy.rotate_left(1).wrapping_add(17);
        } else {
            t.rank = t.rank.saturating_add(15);
            t.energy = t.energy.rotate_right(2).wrapping_add(23);
        }

        // 手動 seeds: [b"side_pot", manager, label] ＋ user_bump（検証と独立）
        let seeds = &[
            b"side_pot".as_ref(),
            t.manager.as_ref(),
            label.as_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let pda = Pubkey::create_program_address(
            &[b"side_pot", t.manager.as_ref(), label.as_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(E::SeedCompute))?;

        let ix = system_instruction::transfer(&pda, &ctx.accounts.recipient.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.side_pot_hint.to_account_info(),
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
    #[account(
        init, payer=manager, space=8+32+8+4+1,
        seeds=[b"treasury", manager.key().as_ref()], bump
    )]
    pub treasury: Account<'info, TreasuryState>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayFromSidePot<'info> {
    #[account(
        mut,
        seeds=[b"treasury", manager.key().as_ref()],
        bump=treasury.bump_main
    )]
    pub treasury: Account<'info, TreasuryState>,
    /// CHECK
    pub side_pot_hint: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TreasuryState {
    pub manager: Pubkey,
    pub energy: u64,
    pub rank: u32,
    pub bump_main: u8,
}

#[error_code]
pub enum E {
    #[msg("missing bump")] MissingBump,
    #[msg("seed compute failed")] SeedCompute,
    #[msg("label too long")] LabelTooLong,
}
