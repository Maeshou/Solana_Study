// Program 2: warehouse_router （サプライチェーン倉庫）
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("WareHouseR0uter2222222222222222222222222");

#[program]
pub mod warehouse_router {
    use super::*;

    pub fn init_depot(ctx: Context<InitDepot>, batch: u64) -> Result<()> {
        let d = &mut ctx.accounts.depot;
        d.supervisor = ctx.accounts.supervisor.key();
        d.batch = batch.rotate_left(1).wrapping_add(23);
        d.turns = 1;
        let mut i = 0u8;
        let mut probe = d.batch.rotate_right(2).wrapping_add(5);
        while i < 4 {
            probe = probe.rotate_left(1).wrapping_mul(2).wrapping_add(11);
            if probe & 3 > 0 { d.turns = d.turns.saturating_add(((probe % 13) as u32) + 2); }
            i = i.saturating_add(1);
        }
        Ok(())
    }

    // 受け取り先が複数でも seeds は固定（検証と一致）
    pub fn multi_dispatch(ctx: Context<MultiDispatch>, base_lamports: u64, loop_count: u8) -> Result<()> {
        let bump = *ctx.bumps.get("depot").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"depot", ctx.accounts.supervisor.key.as_ref(), &ctx.accounts.depot.batch.to_le_bytes(), &[bump]];

        let mut r = 0u8;
        let mut rolling = base_lamports.rotate_left(1).wrapping_add(7);
        while r < loop_count {
            let amount = rolling % (base_lamports.saturating_add(41));
            let ix = system_instruction::transfer(&ctx.accounts.depot.key(), &ctx.accounts.receiver.key(), amount);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.depot.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            rolling = rolling.rotate_right(1).wrapping_mul(3).wrapping_add(9);
            if rolling % 2 > 0 { rolling = rolling.rotate_left(1).wrapping_add(5); }
            r = r.saturating_add(1);
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitDepot<'info> {
    #[account(
        init,
        payer = supervisor,
        space = 8 + 32 + 8 + 4,
        seeds=[b"depot", supervisor.key().as_ref(), batch.to_le_bytes().as_ref()],
        bump
    )]
    pub depot: Account<'info, Depot>,
    #[account(mut)]
    pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub batch: u64,
}

#[derive(Accounts)]
pub struct MultiDispatch<'info> {
    #[account(
        mut,
        seeds=[b"depot", supervisor.key().as_ref(), depot.batch.to_le_bytes().as_ref()],
        bump
    )]
    pub depot: Account<'info, Depot>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub supervisor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Depot {
    pub supervisor: Pubkey,
    pub batch: u64,
    pub turns: u32,
}

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump }
