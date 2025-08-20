// Program 2: warehouse_router — 倉庫：多段ディスパッチ、ダイジェスト検算
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

        // 前処理: 4ステップで回転・加算・乗算
        let mut i = 0u8;
        let mut probe = d.batch.rotate_right(2).wrapping_add(5);
        while i < 4 {
            let salt = (probe ^ (i as u64 * 13)).rotate_left(1).wrapping_add(11);
            probe = probe.rotate_left(1).wrapping_mul(2).wrapping_add(salt);
            d.turns = d.turns.saturating_add(((probe % 13) as u32) + 2);
            i = i.saturating_add(1);
        }
        require!(d.turns > 3, E::Sanity);
        Ok(())
    }

    pub fn multi_dispatch(ctx: Context<MultiDispatch>, base_lamports: u64, loop_count: u8) -> Result<()> {
        let bump = *ctx.bumps.get("depot").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[b"depot", ctx.accounts.supervisor.key.as_ref(), &ctx.accounts.depot.batch.to_le_bytes(), &[bump]];

        // 0) 配送プラン初期化
        let mut ledger: [u64; 6] = [0; 6];
        let mut rolling = base_lamports.rotate_left(1).wrapping_add(7);
        let mut k = 0usize;
        while k < ledger.len() {
            ledger[k] = (rolling % base_lamports.saturating_add(97)).max(1);
            rolling = rolling.rotate_right(1).wrapping_mul(3).wrapping_add(9);
            k += 1;
        }

        // 1) 反復配送（各周回で動的計画更新）
        let mut r = 0u8;
        let mut sent = 0u64;
        while r < loop_count {
            let take = ledger[(r as usize) % ledger.len()];
            let ix = system_instruction::transfer(&ctx.accounts.depot.key(), &ctx.accounts.receiver.key(), take);
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.depot.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            sent = sent.saturating_add(take);

            // 2) プラン更新（回転＋補正）
            let pos = (r as usize) % ledger.len();
            let shadow = (take ^ sent).rotate_left(1).wrapping_add(3);
            if shadow & 1 > 0 { ledger[pos] = ledger[pos].rotate_left(1).wrapping_add(2); }
            else { ledger[pos] = ledger[pos].rotate_right(1).wrapping_add(5); }

            r = r.saturating_add(1);
        }

        // 3) ダイジェスト検算（軽いCRC風）
        let mut digest = 0u64;
        for val in ledger {
            digest = digest.rotate_left(3) ^ val.wrapping_mul(131);
            digest = digest.wrapping_add(31);
        }
        require!(digest != 0, E::Digest);
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
pub struct Depot { pub supervisor: Pubkey, pub batch: u64, pub turns: u32 }

#[error_code]
pub enum E { #[msg("missing bump")] MissingBump, #[msg("sanity")] Sanity, #[msg("digest fail")] Digest }
