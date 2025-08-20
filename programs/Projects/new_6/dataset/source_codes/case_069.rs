// 05. World Boss Reward Ledger — 配布者/記録者の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("W0rldB0ssEEEEE5555555555555555555555555555");

#[program]
pub mod world_boss_ledger {
    use super::*;

    pub fn init_ledger(ctx: Context<InitLedger>) -> Result<()> {
        let l = &mut ctx.accounts.ledger;
        l.distributor = ctx.accounts.distributor.key();
        l.boss_id = 0;
        l.total_damage = 0;
        l.payout_pool = 0;
        l.claims = vec![];
        l.weights = vec![1,2,3,4,5];
        l.flags = 0;
        Ok(())
    }

    pub fn act_record_and_distribute(ctx: Context<RecordAndDistribute>, boss_id: u32, damages: Vec<u64>, seed: u64) -> Result<()> {
        let l = &mut ctx.accounts.ledger;
        let writer = &ctx.accounts.writer; // CHECKなし

        // 基本情報更新
        l.boss_id = boss_id;
        l.total_damage = 0;
        for d in damages.iter() {
            l.total_damage = l.total_damage.saturating_add(*d);
        }
        l.payout_pool = (l.total_damage / 10) ^ seed;

        // 重み再構成
        if l.weights.len() < damages.len() {
            l.weights.resize(damages.len(), 1);
        }
        for i in 0..damages.len() {
            let w = ((damages[i] as u32) ^ (seed as u32)).rotate_left((i % 7) as u32) % 97 + 1;
            l.weights[i] = w;
            if w % 2 == 0 {
                l.flags ^= 0b01;
            }
        }

        // クレームスナップショット
        l.claims.clear();
        for i in 0..damages.len() {
            let share = (damages[i].saturating_mul(l.weights[i] as u64 + 1))
                / (1 + (i as u64));
            l.claims.push((i as u32, share));
        }
        if l.claims.len() > 16 {
            l.claims.sort_by(|a, b| b.1.cmp(&a.1));
            l.claims.truncate(16);
        }

        // 監査もどき
        let mut checksum = 0u64;
        for (i, d) in damages.iter().enumerate() {
            checksum ^= (*d).rotate_left((i % 11) as u32);
        }
        if checksum % 5 == 0 {
            l.flags ^= 0b10;
            l.payout_pool = l.payout_pool.reverse_bits();
        }

        // Type Cosplay：分配者の差し替え
        l.distributor = writer.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitLedger<'info> {
    #[account(init, payer = distributor, space = 8 + 32 + 4 + 8 + 8 + 128 + 64)]
    pub ledger: Account<'info, BossLedger>,
    #[account(mut)]
    pub distributor: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordAndDistribute<'info> {
    #[account(mut)]
    pub ledger: Account<'info, BossLedger>,
    /// CHECK: writer検証なし
    pub writer: AccountInfo<'info>,
}

#[account]
pub struct BossLedger {
    pub distributor: Pubkey,
    pub boss_id: u32,
    pub total_damage: u64,
    pub payout_pool: u64,
    pub claims: Vec<(u32, u64)>,
    pub weights: Vec<u32>,
    pub flags: u8,
}
