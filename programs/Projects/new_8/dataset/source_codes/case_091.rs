use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("MoUnTStAbLeX77777777777777777777777777777");

#[program]
pub mod mount_stable_keeper {
    use super::*;

    pub fn init_stable(ctx: Context<InitStable>, capacity: u64) -> Result<()> {
        let st = &mut ctx.accounts.stable;
        st.owner = ctx.accounts.herder.key();
        st.bump_mem = *ctx.bumps.get("stable").ok_or(error!(EMS::NoBump))?;
        st.feed = capacity.rotate_left(1).wrapping_add(53);
        st.turns = 2;

        // Vec 生成 → while → if
        let mut packs: Vec<u64> = (1..5).map(|i| st.feed.wrapping_mul(i * 5)).collect();
        let mut i = 1u8;
        while i < 4 {
            let v = packs[i as usize - 1].rotate_right(1).wrapping_add(7 + i as u64);
            st.feed = st.feed.wrapping_add(v).wrapping_mul(2);
            st.turns = st.turns.saturating_add(((st.feed % 23) as u32) + 3);
            i = i.saturating_add(1);
        }
        if st.feed > capacity {
            st.feed = st.feed.rotate_left(2).wrapping_add(19);
            st.turns = st.turns.saturating_add(((st.feed % 27) as u32) + 4);
        }
        Ok(())
    }

    pub fn pay_grooming(ctx: Context<PayGrooming>, horse_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let st = &mut ctx.accounts.stable;

        // for → if → while（順序多様化）
        for r in 1..3 {
            let delta = (st.feed ^ (r as u64 * 13)).rotate_left(1);
            st.feed = st.feed.wrapping_add(delta).wrapping_mul(2).wrapping_add(11 + r as u64);
            st.turns = st.turns.saturating_add(((st.feed % 21) as u32) + 3);
        }
        if lamports > 360 {
            let mut u = 1u8;
            let mut x = lamports.rotate_left(2);
            while u < 4 {
                let z = (x ^ (u as u64 * 17)).rotate_right(1);
                x = x.wrapping_add(z);
                st.feed = st.feed.wrapping_add(z).wrapping_mul(3).wrapping_add(9 + u as u64);
                st.turns = st.turns.saturating_add(((st.feed % 25) as u32) + 5);
                u = u.saturating_add(1);
            }
        }

        // BSC: user_bump を seeds に使って署名
        let seeds = &[
            b"groom_fee".as_ref(),
            st.owner.as_ref(),
            &horse_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let purse = Pubkey::create_program_address(
            &[b"groom_fee", st.owner.as_ref(), &horse_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EMS::SeedCompute))?;
        let ix = system_instruction::transfer(&purse, &ctx.accounts.groomer.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.fee_hint.to_account_info(),
                ctx.accounts.groomer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStable<'info> {
    #[account(init, payer=herder, space=8+32+8+4+1, seeds=[b"stable", herder.key().as_ref()], bump)]
    pub stable: Account<'info, StableState>,
    #[account(mut)]
    pub herder: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayGrooming<'info> {
    #[account(mut, seeds=[b"stable", herder.key().as_ref()], bump=stable.bump_mem)]
    pub stable: Account<'info, StableState>,
    /// CHECK
    pub fee_hint: AccountInfo<'info>,
    #[account(mut)]
    pub groomer: AccountInfo<'info>,
    pub herder: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct StableState { pub owner: Pubkey, pub feed: u64, pub turns: u32, pub bump_mem: u8 }
#[error_code] pub enum EMS { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
