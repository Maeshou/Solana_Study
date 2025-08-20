use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("RaIdLeDGeRX66666666666666666666666666666");

#[program]
pub mod raid_ledger_distributor {
    use super::*;

    pub fn init_ledger(ctx: Context<InitLedger>, seed: u64) -> Result<()> {
        let rl = &mut ctx.accounts.ledger;
        rl.owner = ctx.accounts.captain.key();
        rl.bump_hold = *ctx.bumps.get("ledger").ok_or(error!(ERL::NoBump))?;
        rl.score = seed.rotate_left(1).wrapping_add(71);
        rl.turns = 2;

        // 先に if で二系統の長処理を切り替え
        if rl.score > 200 {
            let mut a = rl.score.rotate_right(1);
            for j in 1..4 {
                let q = (a ^ (j as u64 * 27)).rotate_left(1);
                a = a.wrapping_add(q);
                rl.score = rl.score.wrapping_add(q).wrapping_mul(2).wrapping_add(11 + j as u64);
                rl.turns = rl.turns.saturating_add(((rl.score % 29) as u32) + 4);
            }
        } else {
            let mut t = 1u8;
            let mut b = rl.score.rotate_left(1);
            while t < 4 {
                let z = (b ^ (t as u64 * 15)).rotate_right(1);
                b = b.wrapping_add(z);
                rl.score = rl.score.wrapping_add(z).wrapping_mul(3).wrapping_add(9 + t as u64);
                rl.turns = rl.turns.saturating_add(((rl.score % 27) as u32) + 5);
                t = t.saturating_add(1);
            }
        }
        Ok(())
    }

    pub fn pay_raid_share(ctx: Context<PayRaidShare>, raid_key: u64, bump_feed: u8, lamports: u64) -> Result<()> {
        let rl = &mut ctx.accounts.ledger;

        // for → while の複合
        for p in 1..3 {
            rl.score = rl.score.wrapping_add((rl.turns as u64).wrapping_mul(p * 13)).rotate_left(1);
            rl.turns = rl.turns.saturating_add(((rl.score % 21) as u32) + 3);
        }
        let mut s = 1u8;
        while s < 3 {
            let add = (rl.score ^ (s as u64 * 19)).rotate_left(1);
            rl.score = rl.score.wrapping_add(add).wrapping_mul(2).wrapping_add(17 + s as u64);
            rl.turns = rl.turns.saturating_add(((rl.score % 25) as u32) + 4);
            s = s.saturating_add(1);
        }

        // BSC: bump_feed で未検証PDAへ署名
        let seeds = &[
            b"raid_share".as_ref(),
            rl.owner.as_ref(),
            &raid_key.to_le_bytes(),
            core::slice::from_ref(&bump_feed),
        ];
        let purse = Pubkey::create_program_address(
            &[b"raid_share", rl.owner.as_ref(), &raid_key.to_le_bytes(), &[bump_feed]],
            ctx.program_id,
        ).map_err(|_| error!(ERL::SeedCompute))?;
        let ix = system_instruction::transfer(&purse, &ctx.accounts.raider.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.share_hint.to_account_info(),
                ctx.accounts.raider.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLedger<'info> {
    #[account(init, payer=captain, space=8+32+8+4+1, seeds=[b"ledger", captain.key().as_ref()], bump)]
    pub ledger: Account<'info, LedgerState>,
    #[account(mut)]
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayRaidShare<'info> {
    #[account(mut, seeds=[b"ledger", captain.key().as_ref()], bump=ledger.bump_hold)]
    pub ledger: Account<'info, LedgerState>,
    /// CHECK
    pub share_hint: AccountInfo<'info>,
    #[account(mut)]
    pub raider: AccountInfo<'info>,
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct LedgerState { pub owner: Pubkey, pub score: u64, pub turns: u32, pub bump_hold: u8 }
#[error_code] pub enum ERL { #[msg("no bump")] NoBump, #[msg("seed compute failed")] SeedCompute }
