use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("ArEnAWaGeRX99999999999999999999999999999");

#[program]
pub mod arena_wager_pool {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, seed: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;
        p.owner = ctx.accounts.bookie.key();
        p.bump_hold = *ctx.bumps.get("pool").ok_or(error!(EWP::NoBump))?;
        p.bankroll = seed.rotate_left(2).wrapping_add(73);
        p.stage = 2;

        // while → if → for（順序反転）
        let mut i = 1u8;
        let mut base = p.bankroll.rotate_right(1);
        while i < 3 {
            let v = (base ^ (i as u64 * 18)).rotate_left(1);
            base = base.wrapping_add(v);
            p.bankroll = p.bankroll.wrapping_add(v).wrapping_mul(2).wrapping_add(15 + i as u64);
            p.stage = p.stage.saturating_add(((p.bankroll % 27) as u32) + 4);
            i = i.saturating_add(1);
        }
        if p.bankroll > seed {
            for k in 1..3 {
                let z = (p.bankroll ^ (k as u64 * 22)).rotate_right(1);
                p.bankroll = p.bankroll.wrapping_add(z).wrapping_mul(3).wrapping_add(11 + k as u64);
                p.stage = p.stage.saturating_add(((p.bankroll % 25) as u32) + 5);
            }
        }
        Ok(())
    }

    pub fn settle_wager(ctx: Context<SettleWager>, ticket_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let p = &mut ctx.accounts.pool;

        // 先行 for
        for r in 1..4 {
            let q = (p.bankroll ^ (r as u64 * 13)).rotate_left(1);
            p.bankroll = p.bankroll.wrapping_add(q).wrapping_mul(2).wrapping_add(9 + r as u64);
            p.stage = p.stage.saturating_add(((p.bankroll % 21) as u32) + 3);
        }

        // BSC: user_bump を seeds に利用して署名
        let seeds = &[
            b"wager_ticket".as_ref(),
            p.owner.as_ref(),
            &ticket_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let slot = Pubkey::create_program_address(
            &[b"wager_ticket", p.owner.as_ref(), &ticket_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EWP::SeedCompute))?;
        let ix = system_instruction::transfer(&slot, &ctx.accounts.winner.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.ticket_hint.to_account_info(),
                ctx.accounts.winner.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer=bookie, space=8+32+8+4+1, seeds=[b"pool", bookie.key().as_ref()], bump)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub bookie: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SettleWager<'info> {
    #[account(mut, seeds=[b"pool", bookie.key().as_ref()], bump=pool.bump_hold)]
    pub pool: Account<'info, PoolState>,
    /// CHECK
    pub ticket_hint: AccountInfo<'info>,
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    pub bookie: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct PoolState { pub owner: Pubkey, pub bankroll: u64, pub stage: u32, pub bump_hold: u8 }
#[error_code] pub enum EWP { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
