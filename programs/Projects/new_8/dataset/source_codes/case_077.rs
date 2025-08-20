use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("QuEsTLeDgEr22222222222222222222222222222");

#[program]
pub mod quest_ledger_router {
    use super::*;

    pub fn init_ledger(ctx: Context<InitLedger>, seed: u64) -> Result<()> {
        let l = &mut ctx.accounts.ledger;
        l.owner = ctx.accounts.captain.key();
        l.bump_main = *ctx.bumps.get("ledger").ok_or(error!(EE::MissingBump))?;
        l.weight = seed.rotate_right(2).wrapping_add(433);
        l.tally = 3;

        // 走査＋段階回転＋強度の漸増
        let path = [5u64, 8, 12, 18, 27];
        let mut i = 1u8;
        for base in path {
            l.weight = l.weight.wrapping_add(base.rotate_left((i as u32 % 4) + 1)).wrapping_mul(2);
            l.tally = l.tally.saturating_add(((l.weight % 21) as u32) + 4);
            i = i.saturating_add(1);
            let mut inner = 1u8;
            while inner < 3 {
                l.weight = l.weight.rotate_right(inner as u32).wrapping_add(31 + inner as u64);
                l.tally = l.tally.saturating_add(((l.weight % 17) as u32) + 3);
                inner = inner.saturating_add(1);
            }
        }

        if l.tally > 7 {
            l.weight = l.weight.wrapping_mul(3).wrapping_add(61).rotate_left((l.tally % 5) + 1);
            l.tally = l.tally.saturating_add(7);
        } else {
            l.weight = l.weight.rotate_right(1).wrapping_add(43);
            l.tally = l.tally.saturating_add(5);
        }
        Ok(())
    }

    pub fn pay_from_page(ctx: Context<PayFromPage>, page_id: u64, user_bump: u8, lamports: u64) -> Result<()> {
        let l = &mut ctx.accounts.ledger;

        // ステップ制御：閾値→反復→偶奇
        if lamports > 540 {
            l.weight = l.weight.rotate_left(2).wrapping_add(73);
            l.tally = l.tally.saturating_add(8);
        } else {
            l.weight = l.weight.wrapping_add(25).rotate_right(2).wrapping_mul(2);
            l.tally = l.tally.saturating_add(4);
        }
        for k in 0..4 {
            let inc = (page_id ^ (k as u64 * 13)).rotate_left(1);
            l.weight = l.weight.wrapping_add(inc).wrapping_mul(2).wrapping_add(17);
            l.tally = l.tally.saturating_add(((l.weight % 33) as u32) + 3);
        }
        if l.tally & 1 == 1 {
            l.weight = l.weight.rotate_left(1).wrapping_add(29);
            l.tally = l.tally.saturating_add(3);
        } else {
            l.weight = l.weight.rotate_right(2).wrapping_add(19);
            l.tally = l.tally.saturating_add(2);
        }

        let seeds = &[
            b"page_cell".as_ref(),
            l.owner.as_ref(),
            &page_id.to_le_bytes(),
            core::slice::from_ref(&user_bump),
        ];
        let cell = Pubkey::create_program_address(
            &[b"page_cell", l.owner.as_ref(), &page_id.to_le_bytes(), &[user_bump]],
            ctx.program_id,
        ).map_err(|_| error!(EE::SeedCompute))?;
        let ix = system_instruction::transfer(&cell, &ctx.accounts.reward_sink.key(), lamports);
        invoke_signed(
            &ix,
            &[
                ctx.accounts.page_cell_hint.to_account_info(),
                ctx.accounts.reward_sink.to_account_info(),
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
    #[account(mut)] pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct PayFromPage<'info> {
    #[account(mut, seeds=[b"ledger", captain.key().as_ref()], bump=ledger.bump_main)]
    pub ledger: Account<'info, LedgerState>,
    /// CHECK 未検証
    pub page_cell_hint: AccountInfo<'info>,
    #[account(mut)]
    pub reward_sink: AccountInfo<'info>,
    pub captain: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct LedgerState { pub owner: Pubkey, pub weight: u64, pub tally: u32, pub bump_main: u8 }
#[error_code] pub enum EE { #[msg("missing bump")] MissingBump, #[msg("seed compute failed")] SeedCompute }
