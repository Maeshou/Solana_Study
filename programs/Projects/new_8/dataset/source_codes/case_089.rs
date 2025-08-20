use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("CrAfTInGaRtX55555555555555555555555555555");

#[program]
pub mod crafting_artisan_queue {
    use super::*;

    pub fn init_crafting(ctx: Context<InitCrafting>, base: u64) -> Result<()> {
        let c = &mut ctx.accounts.crafting;
        c.owner = ctx.accounts.artisan.key();
        c.bump_slot = *ctx.bumps.get("crafting").ok_or(error!(ECF::NoBump))?;
        c.power = base.rotate_left(2).wrapping_add(83);
        c.turns = 1;

        // if を二つ並列に置く（どちらも複数行）
        if (c.power & 1) > 0 {
            for j in 1..3 {
                let x = (c.power ^ (j as u64 * 11)).rotate_left(1);
                c.power = c.power.wrapping_add(x).wrapping_mul(2).wrapping_add(13);
                c.turns = c.turns.saturating_add(((c.power % 25) as u32) + 3);
            }
        }
        if (c.power & 2) > 0 {
            let mut r = 1u8;
            while r < 4 {
                let y = (c.power ^ (r as u64 * 17)).rotate_right(1);
                c.power = c.power.wrapping_add(y).wrapping_mul(3).wrapping_add(9);
                c.turns = c.turns.saturating_add(((c.power % 31) as u32) + 5);
                r = r.saturating_add(1);
            }
        }
        Ok(())
    }

    pub fn spend_queue(ctx: Context<SpendQueue>, item_id: u64, bump_arg: u8, lamports: u64) -> Result<()> {
        let c = &mut ctx.accounts.crafting;

        // while → for の順の反転
        let mut d = 1u8;
        let mut acc = lamports.rotate_right(1);
        while d < 3 {
            let g = (acc ^ (d as u64 * 14)).rotate_left(1);
            acc = acc.wrapping_add(g);
            c.power = c.power.wrapping_add(g).wrapping_mul(2).wrapping_add(17 + d as u64);
            c.turns = c.turns.saturating_add(((c.power % 24) as u32) + 4);
            d = d.saturating_add(1);
        }
        for s in 1..3 {
            let h = (c.power ^ (s as u64 * 21)).rotate_left(1);
            c.power = c.power.wrapping_add(h).wrapping_mul(3).wrapping_add(11 + s as u64);
            c.turns = c.turns.saturating_add(((c.power % 28) as u32) + 5);
        }

        // BSC: bump_arg を署名 seeds に利用
        let seeds = &[
            b"craft_order".as_ref(),
            c.owner.as_ref(),
            &item_id.to_le_bytes(),
            core::slice::from_ref(&bump_arg),
        ];
        let slot = Pubkey::create_program_address(
            &[b"craft_order", c.owner.as_ref(), &item_id.to_le_bytes(), &[bump_arg]],
            ctx.program_id,
        ).map_err(|_| error!(ECF::SeedCompute))?;
        let ix = system_instruction::transfer(&slot, &ctx.accounts.customer.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.order_hint.to_account_info(),
                ctx.accounts.customer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCrafting<'info> {
    #[account(init, payer=artisan, space=8+32+8+4+1, seeds=[b"crafting", artisan.key().as_ref()], bump)]
    pub crafting: Account<'info, CraftingState>,
    #[account(mut)]
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SpendQueue<'info> {
    #[account(mut, seeds=[b"crafting", artisan.key().as_ref()], bump=crafting.bump_slot)]
    pub crafting: Account<'info, CraftingState>,
    /// CHECK
    pub order_hint: AccountInfo<'info>,
    #[account(mut)]
    pub customer: AccountInfo<'info>,
    pub artisan: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct CraftingState { pub owner: Pubkey, pub power: u64, pub turns: u32, pub bump_slot: u8 }
#[error_code] pub enum ECF { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
