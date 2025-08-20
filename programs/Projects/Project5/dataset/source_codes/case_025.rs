// ============================================================================
// 1) Gem Crafter — ジェム加工（PDAあり・イベント・ヘルパ関数・Keccak）
//    防止: seeds一意化 + has_one + constraint + assert_ne!
// ============================================================================
declare_id!("GEMC11111111111111111111111111111111");
use anchor_lang::prelude::*;
use solana_program::keccak::{hashv};

#[event]
pub struct CraftEvent {
    pub owner: Pubkey,
    pub passes: u8,
    pub rarity_delta: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BenchMode { Idle, Warm, Overdrive }

#[program]
pub mod gem_crafter {
    use super::*;

    pub fn init_bench(ctx: Context<InitBench>, limit: u32) -> Result<()> {
        let accs = &mut ctx.accounts;
        accs.bench.owner = accs.crafter.key();
        accs.bench.limit = limit;
        accs.bench.mode = BenchMode::Warm;
        // bin/ledger はゼロ初期化に任せる
        Ok(())
    }

    pub fn craft(ctx: Context<Craft>, passes: u8) -> Result<()> {
        // ctx直書きは最小化。ローカル束縛＋タプルでまとめて扱う。
        let accs = &mut ctx.accounts;
        let (bench, bin, ledger) = (&mut accs.bench, &mut accs.ore_bin, &mut accs.ledger);

        // 追加の実行時ガード（属性でも守っているが二重に）
        assert_ne!(bench.key(), bin.key(), "bench/bin must differ");

        // ループ相当の処理はヘルパに切り出し
        let rarity_delta = simulate_crafting(bin, passes);
        ledger.rarity_score = ledger.rarity_score.saturating_add(rarity_delta);
        ledger.crafts = ledger.crafts.saturating_add(passes as u64);

        // Keccakで決定論的な「色合いシード」を混ぜる（非乱数）
        let hue_boost = hue_mix(&[bench.owner.as_ref(), &passes.to_le_bytes(), &ledger.crafts.to_le_bytes()]);
        bin.chromatic = bin.chromatic.saturating_add(hue_boost as u32);

        // 分岐（各ブロック4行以上）
        if bin.iron.saturating_add(bin.silica) > bench.limit {
            bench.mode = BenchMode::Overdrive;
            ledger.rarity_score = ledger.rarity_score.saturating_add(7);
            bin.hardness = bin.hardness.saturating_add(5);
            msg!("limit exceeded: overdrive on, rarity+7 hardness+5");
        } else {
            bench.mode = BenchMode::Warm;
            ledger.crafts = ledger.crafts.saturating_add(1);
            bin.hardness = bin.hardness.saturating_add(2);
            msg!("within limit: warm mode, crafts+1 hardness+2");
        }

        emit!(CraftEvent { owner: bench.owner, passes, rarity_delta });
        Ok(())
    }
}

// ---- ヘルパ：ロジックは関数へ（テストもしやすい）
fn simulate_crafting(bin: &mut OreBin, passes: u8) -> u64 {
    let mut rarity = 0u64;
    for step in 0..passes {
        let gain = (2 + (step % 3)) as u32;
        bin.iron = bin.iron.saturating_add(gain);
        bin.silica = bin.silica.saturating_add(gain + 1);
        bin.cutlines = bin.cutlines.saturating_add(3);
        rarity = rarity.saturating_add((gain as u64) * 2);
    }
    rarity
}

fn hue_mix(parts: &[&[u8]]) -> u16 {
    let h = hashv(parts);
    // 0/1代入の代わりにハッシュ下位ビットから安定的な値を抽出
    u16::from_le_bytes([h.0[0], h.0[1]]) % 97 + 3 // 3..=99
}

#[derive(Accounts)]
pub struct InitBench<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 4 + 1)]
    pub bench: Account<'info, Bench>,
    #[account(init, payer = payer, space = 8 + 4 + 4 + 4 + 4)]
    pub ore_bin: Account<'info, OreBin>,
    #[account(init, payer = payer, space = 8 + 8 + 8, seeds=[b"ledger", crafter.key().as_ref()], bump)]
    pub ledger: Account<'info, Ledger>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub crafter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Craft<'info> {
    #[account(mut, has_one = owner)]
    pub bench: Account<'info, Bench>,
    #[account(mut, constraint = ore_bin.key() != ledger.key(), error = GemErr::Dup)]
    pub ore_bin: Account<'info, OreBin>,
    #[account(mut, seeds=[b"ledger", owner.key().as_ref()], bump)]
    pub ledger: Account<'info, Ledger>,
    pub owner: Signer<'info>,
}

#[account] pub struct Bench { pub owner: Pubkey, pub limit: u32, pub mode: BenchMode }
#[account] pub struct OreBin { pub iron: u32, pub silica: u32, pub cutlines: u32, pub hardness: u32, pub chromatic: u32 }
#[account] pub struct Ledger { pub crafts: u64, pub rarity_score: u64 }

#[error_code] pub enum GemErr { #[msg("duplicate mutable account")] Dup }

