// 06. Seasonal Ranking Board — 管理者/更新者の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("S34s0nB04rdFFFF6666666666666666666666666666");

#[program]
pub mod seasonal_ranking {
    use super::*;

    pub fn init_board(ctx: Context<InitBoard>, season: u16) -> Result<()> {
        let b = &mut ctx.accounts.board;
        b.admin = ctx.accounts.admin.key();
        b.season = season;
        b.entries = vec![];
        b.snapshot = vec![];
        b.flags = 0;
        b.seed = 1;
        Ok(())
    }

    pub fn act_update(ctx: Context<UpdateBoard>, scores: Vec<(Pubkey, u64)>, salt: u64) -> Result<()> {
        let b = &mut ctx.accounts.board;
        let updater = &ctx.accounts.updater; // CHECKなし

        // 取り込み
        for (k, s) in scores.iter() {
            b.entries.push((*k, *s));
        }
        if b.entries.len() > 64 {
            b.entries.drain(0..(b.entries.len() - 64));
        }

        // 正規化＋ハッシュ
        let mut mixed = 0u64;
        for i in 0..b.entries.len() {
            mixed ^= b.entries[i].1.rotate_left((i % 17) as u32) ^ salt;
        }
        b.seed = b.seed.rotate_left(((mixed % 31) as u32) + 1);

        // スナップショット
        b.snapshot = b.entries.clone();
        b.snapshot.sort_by(|a, b| b.1.cmp(&a.1));
        if b.snapshot.len() > 20 {
            b.snapshot.truncate(20);
        }

        // モード切替
        if mixed % 9 == 0 {
            b.flags ^= 0b001;
            b.snapshot.reverse();
        }
        if mixed.trailing_zeros() > 6 {
            b.flags ^= 0b010;
            b.entries.rotate_left(3);
        }

        // Type Cosplay：updaterをadminに
        b.admin = updater.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBoard<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 2 + 256 + 256 + 1 + 8)]
    pub board: Account<'info, Board>,
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateBoard<'info> {
    #[account(mut)]
    pub board: Account<'info, Board>,
    /// CHECK: 役割検証なし
    pub updater: AccountInfo<'info>,
}

#[account]
pub struct Board {
    pub admin: Pubkey,
    pub season: u16,
    pub entries: Vec<(Pubkey, u64)>,
    pub snapshot: Vec<(Pubkey, u64)>,
    pub flags: u8,
    pub seed: u64,
}
