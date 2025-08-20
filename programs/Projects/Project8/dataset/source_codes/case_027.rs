// B) warehouse_router_v2 — 擬似シャッフル順で配布（インデックス列を先に permute）
//    * while を避け、for + loop を混在。送金は permuted index の順序で実行。
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

declare_id!("WareHouseR0uterV22222222222222222222222");

#[program]
pub mod warehouse_router_v2 {
    use super::*;

    pub fn init_depot(ctx: Context<InitDepot>, batch: u64) -> Result<()> {
        let d = &mut ctx.accounts.depot;
        d.supervisor = ctx.accounts.supervisor.key();
        d.batch = batch.rotate_left(1).wrapping_add(23);
        d.turns = 1;

        // ここも while ではなく for で揺らす
        for i in 0u8..6 {
            let salt = (d.batch ^ (i as u64 * 13)).rotate_left(1).wrapping_add(11);
            let probe = d.batch.rotate_left(1).wrapping_mul(2).wrapping_add(salt);
            d.turns = d.turns.saturating_add(((probe % 13) as u32) + 2);
        }
        require!(d.turns > 3, E::Sanity);
        Ok(())
    }

    pub fn multi_dispatch(ctx: Context<MultiDispatch>, base_lamports: u64, loop_count: u8) -> Result<()> {
        let bump = *ctx.bumps.get("depot").ok_or(error!(E::MissingBump))?;
        let seeds: &[&[u8]] = &[
            b"depot",
            ctx.accounts.supervisor.key.as_ref(),
            &ctx.accounts.depot.batch.to_le_bytes(),
            &[bump],
        ];

        // 0) 配送金額テーブル（for）
        let mut amounts: [u64; 8] = [0; 8];
        let mut rolling = base_lamports.rotate_left(1).wrapping_add(7);
        for i in 0..amounts.len() {
            amounts[i] = (rolling % base_lamports.saturating_add(97)).max(1);
            rolling = rolling.rotate_right(1).wrapping_mul(3).wrapping_add(9);
        }

        // 1) インデックスの擬似シャッフル列を作る（Fisher-Yates風だが while ではなく loop+break）
        let mut idx: [usize; 8] = [0,1,2,3,4,5,6,7];
        let mut pos = idx.len();
        loop {
            if pos == 0 { break; }
            pos -= 1;
            // 疑似乱数は rolling から生成
            let r = (rolling ^ (pos as u64 * 131)).rotate_left(2) as usize;
            let pick = r % (pos + 1);
            let tmp = idx[pos];
            idx[pos] = idx[pick];
            idx[pick] = tmp;
            rolling = rolling.rotate_left(3).wrapping_add(31);
        }

        // 2) 送金は「シャッフル済み idx 順」で for 実行（while順固定を崩す）
        let mut sent = 0u64;
        let mut rounds = 0u8;
        for k in 0..idx.len() {
            if rounds >= loop_count { break; }
            let a = amounts[idx[k]];
            let ix = system_instruction::transfer(
                &ctx.accounts.depot.key(),
                &ctx.accounts.receiver.key(),
                a,
            );
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.depot.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            sent = sent.saturating_add(a);
            rounds = rounds.saturating_add(1);
        }

        // 3) 残り回数があれば loop で再周回（break 抜け）
        let mut cursor = 0usize;
        loop {
            if rounds >= loop_count { break; }
            if cursor >= idx.len() { cursor = 0; }
            let a = amounts[idx[cursor]].rotate_left(1).wrapping_add(3);
            let ix = system_instruction::transfer(
                &ctx.accounts.depot.key(),
                &ctx.accounts.receiver.key(),
                a,
            );
            invoke_signed(
                &ix,
                &[
                    ctx.accounts.depot.to_account_info(),
                    ctx.accounts.receiver.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            sent = sent.saturating_add(a);
            rounds = rounds.saturating_add(1);
            cursor = cursor.saturating_add(1);
        }

        // 4) 検算（for）
        let mut digest = 0u64;
        for v in amounts {
            digest = digest.rotate_left(3) ^ v.wrapping_mul(131);
            digest = digest.wrapping_add(37);
        }
        require!(digest != 0, E::Digest);
        require!(sent > 0, E::SentZero);
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
pub enum E {
    #[msg("missing bump")] MissingBump,
    #[msg("sanity")] Sanity,
    #[msg("digest fail")] Digest,
    #[msg("sent is zero")] SentZero,
}
