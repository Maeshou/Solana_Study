// 02. Energy Forge — 生成者/監督者の混同（Type Cosplay）
use anchor_lang::prelude::*;

declare_id!("En3rgyF0rg3BBBB2222222222222222222222222222");

#[program]
pub mod energy_forge {
    use super::*;

    pub fn init_forge(ctx: Context<InitForge>, seed: u64) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        f.operator = ctx.accounts.operator.key();
        f.seed = seed;
        f.banks = vec![0; 6];
        f.modes = [0u8; 4];
        f.paused = false;
        f.history = vec![];
        f.eff_index = 0;
        Ok(())
    }

    pub fn act_cycle(ctx: Context<CycleForge>, pulses: u16, tweak: u8, tag: String) -> Result<()> {
        let f = &mut ctx.accounts.forge;
        let inspector = &ctx.accounts.inspector; // CHECKなし

        // 履歴追加とトリミング
        let ts = Clock::get()?.unix_timestamp;
        f.history.push(format!("p={}/{} t={}", pulses, tweak, ts));
        if f.history.len() > 30 {
            f.history.drain(0..5);
        }

        // 複合サイクル：banksとmodesを再構成
        let mut i = 0u16;
        while i < pulses {
            let pos = (i as usize) % f.banks.len();
            let shifted = (pulses ^ (i.rotate_left(2))) as u64;
            f.banks[pos] = f.banks[pos].wrapping_add((shifted & 0x3FF) + (tweak as u64));
            if f.banks[pos] & 1 == 1 {
                f.modes[pos % 4] = f.modes[pos % 4].wrapping_add((tweak ^ (pos as u8)) & 0x1F);
                f.seed = f.seed.rotate_left(((pos as u32) & 7) + 1);
            }
            if f.modes[pos % 4] > 127 {
                f.modes[pos % 4] = f.modes[pos % 4] % 101;
                f.banks[pos] = f.banks[pos].reverse_bits();
            }
            i = i.saturating_add(3);
        }

        // タグに応じて効率係数とフラグ更新
        let mut local = 0u32;
        for ch in tag.as_bytes() {
            local = local.wrapping_add(*ch as u32);
            if ch % 2 == 0 {
                f.eff_index = f.eff_index.wrapping_add((ch % 7) as u8);
                f.seed ^= (*ch as u64) << (ch % 8);
            }
        }
        if local % 5 == 0 {
            f.paused = true;
            f.history.push("eff pause".into());
            f.banks.rotate_left(1);
        }
        if f.paused {
            f.modes.iter_mut().for_each(|m| *m = m.rotate_left(1));
            let s = f.banks.iter().fold(0u64, |a, b| a.wrapping_add(*b));
            f.history.push(format!("cooldown={}", s & 0xFF));
        }

        // Type Cosplay：監督者がoperatorにすり替わる
        f.operator = inspector.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitForge<'info> {
    #[account(init, payer = operator, space = 8 + 32 + 8 + 64 + 8 + 64 + 64)]
    pub forge: Account<'info, Forge>,
    #[account(mut)]
    pub operator: AccountInfo<'info>, // 未検証
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CycleForge<'info> {
    #[account(mut)]
    pub forge: Account<'info, Forge>,
    /// CHECK: 役割検証なし
    pub inspector: AccountInfo<'info>,
}

#[account]
pub struct Forge {
    pub operator: Pubkey,
    pub seed: u64,
    pub banks: Vec<u64>,
    pub modes: [u8; 4],
    pub paused: bool,
    pub history: Vec<String>,
    pub eff_index: u8,
}
