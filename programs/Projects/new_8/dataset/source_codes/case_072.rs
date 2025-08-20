// 例5) Craft Station + Kit Slot（手動 seeds: [b"kit_slot", crafter, tag] + user_bump）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("CrAfTStAtIoN0000000000000000000000000005");

#[program]
pub mod craft_station_kitslot {
    use super::*;

    pub fn boot(ctx: Context<Boot>, seed: u64) -> Result<()> {
        let c = &mut ctx.accounts.craft;
        c.crafter = ctx.accounts.crafter.key();
        c.bump_main = *ctx.bumps.get("craft").ok_or(error!(C::MissingBump))?;
        c.tempo = seed % 750 + 120;
        c.grade = 1;

        // if → for → while
        if c.tempo & 1 == 1 {
            c.grade = c.grade.saturating_add(7);
            c.tempo = c.tempo.rotate_left(1).wrapping_add(11);
        } else {
            c.grade = c.grade.saturating_add(9);
            c.tempo = c.tempo.rotate_right(2).wrapping_add(15);
        }

        for i in 0..4 {
            c.tempo = c.tempo.wrapping_add((i as u64) * 19 + 3);
            c.grade = c.grade.saturating_add(((c.tempo % 27) as u32) + 4);
        }

        let mut t = 0u8;
        while t < 2 {
            c.tempo = c.tempo.wrapping_mul(2).wrapping_add((t as u64) * 13 + 9);
            c.grade = c.grade.saturating_add(5 + t as u32);
            t = t.saturating_add(1);
        }
        Ok(())
    }

    pub fn spend_from_kit(ctx: Context<SpendFromKit>, tag: String, user_bump: u8, lamports: u64) -> Result<()> {
        let c = &mut ctx.accounts.craft;
        require!(tag.as_bytes().len() <= 32, C::TagTooLong);

        // while → if
        let mut s = 0u8;
        while s < 3 {
            c.tempo = c.tempo.wrapping_add((s as u64) * 23 + 7);
            c.grade = c.grade.saturating_add(((lamports % 21) as u32) + s as u32 + 6);
            s = s.saturating_add(1);
        }
        if c.grade & 1 == 0 {
            c.tempo = c.tempo.rotate_left(2).wrapping_add(25);
            c.grade = c.grade.saturating_add(12);
        } else {
            c.tempo = c.tempo.rotate_right(1).wrapping_add(17);
            c.grade = c.grade.saturating_add(8);
        }

        let seeds = &[
            b"kit_slot".as_ref(),
            c.crafter.as_ref(),
            tag.as_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let kit = Pubkey::create_program_address(
            &[b"kit_slot", c.crafter.as_ref(), tag.as_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(C::SeedCompute))?;

        let ix = system_instruction::transfer(&kit, &ctx.accounts.beneficiary.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.kit_hint.to_account_info(),
                ctx.accounts.beneficiary.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Boot<'info> {
    #[account(init, payer=crafter, space=8+32+8+4+1, seeds=[b"craft", crafter.key().as_ref()], bump)]
    pub craft: Account<'info, Craft>,
    #[account(mut)]
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SpendFromKit<'info> {
    #[account(mut, seeds=[b"craft", crafter.key().as_ref()], bump=craft.bump_main)]
    pub craft: Account<'info, Craft>,
    /// CHECK
    pub kit_hint: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Craft { pub crafter: Pubkey, pub tempo: u64, pub grade: u32, pub bump_main: u8 }

#[error_code]
pub enum C { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute, #[msg("tag too long")] TagTooLong }
