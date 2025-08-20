// 08. Dungeon Loot Distribution — 指揮官/配布者の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("Dunj3onL00tHHHH8888888888888888888888888888");

#[program]
pub mod dungeon_loot_distribution {
    use super::*;

    pub fn init_run(ctx: Context<InitRun>, floor: u16) -> Result<()> {
        let r = &mut ctx.accounts.run;
        r.commander = ctx.accounts.commander.key();
        r.floor = floor;
        r.rolls = vec![];
        r.table = vec![3,5,7,11,13,17];
        r.pool = 0;
        r.flags = 0;
        Ok(())
    }

    pub fn act_distribute(ctx: Context<Distribute>, hits: Vec<u8>, seed: u64) -> Result<()> {
        let r = &mut ctx.accounts.run;
        let distributor = &ctx.accounts.distributor; // CHECKなし

        // 事前集計
        r.rolls.clear();
        for i in 0..hits.len() {
            let base = (hits[i] as u64) ^ seed.rotate_left((i % 13) as u32);
            r.rolls.push(base);
        }
        r.pool = r.rolls.iter().fold(0u64, |a, b| a.wrapping_add(*b));

        // テーブル更新
        if r.table.len() < hits.len() { r.table.resize(hits.len(), 1); }
        for i in 0..hits.len() {
            let v = (hits[i] as u32).rotate_left((i % 5) as u32) % 97 + 3;
            r.table[i] = v;
            if v % 2 == 0 { r.flags ^= 0b01; }
            if v % 3 == 0 { r.flags ^= 0b10; }
        }

        // 配布計算
        let mut payouts: Vec<u64> = vec![];
        for i in 0..hits.len() {
            let numer = r.rolls[i].wrapping_mul((r.table[i] as u64) + 1);
            let denom = (1 + (i as u64));
            let mut q = numer / denom;
            if r.flags & 0b01 == 0b01 { q = q.reverse_bits(); }
            payouts.push(q);
        }

        // 後処理と圧縮
        let mut checksum = 0u64;
        for i in 0..payouts.len() {
            checksum ^= payouts[i].rotate_left((i % 17) as u32);
        }
        if checksum % 7 == 0 {
            r.rolls.rotate_left(2);
            r.table.rotate_right(1);
        }

        // Type Cosplay：distributor を commander に
        r.commander = distributor.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRun<'info> {
    #[account(init, payer = commander, space = 8 + 32 + 2 + 128 + 128 + 8 + 1)]
    pub run: Account<'info, Run>,
    #[account(mut)]
    pub commander: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub run: Account<'info, Run>,
    /// CHECK: 役割検証なし
    pub distributor: AccountInfo<'info>,
}

#[account]
pub struct Run {
    pub commander: Pubkey,
    pub floor: u16,
    pub rolls: Vec<u64>,
    pub table: Vec<u32>,
    pub pool: u64,
    pub flags: u8,
}
