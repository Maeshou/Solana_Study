use anchor_lang::prelude::*;

declare_id!("B2rYtUvWxZaBcDeFgHiJkLmNoPqRsTuVwXyZaBcDe");

#[program]
pub mod nft_breeding_complex {
    use super::*;

    /// parent_1 と parent_2 から offspring を生成するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn breed_nfts(
        ctx: Context<BreedNfts>,
        seed: u8,
    ) -> ProgramResult {
        let parent_1 = &mut ctx.accounts.parent_1;
        let parent_2 = &mut ctx.accounts.parent_2;
        let offspring = &mut ctx.accounts.offspring;
        let ts = ctx.accounts.clock.unix_timestamp;

        // ❌ 本来はここでキーの不一致を保証すべき
        // require!(
        //     parent_1.key() != parent_2.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // レベルを掛け合わせてキャップを超えたら最大値
        let lvl = (parent_1.level as u16).saturating_mul(parent_2.level as u16);
        offspring.level = if lvl > 255 { 255 } else { lvl as u8 };

        // 経験値に割合を乗じる（オーバーフローは u64::MAX）
        offspring.xp = parent_1
            .xp
            .saturating_mul((seed as u64).saturating_add(1))
            .saturating_div(3);

        // スコアを乗算後にスケーリング
        offspring.score = parent_1
            .score
            .saturating_mul(parent_2.score)
            .saturating_div(1000);

        // trait_id をシフトと XOR でミックス
        offspring.trait_id = (parent_1.trait_id << (seed % 5)) 
            ^ (parent_2.trait_id >> ((seed as u32) % 3));

        // 名前をフォーマットで構築
        offspring.name = format!(
            "{}▶{}#{}",
            parent_1.name,
            parent_2.name,
            seed
        );

        // 混合パラメータを算出（16-bit）
        offspring.mixed = ((parent_1.mixed as u32)
            .saturating_mul(parent_2.mixed as u32)
            .saturating_div(256)) as u16;

        // 操作履歴をクリアして記録
        offspring.history.clear();
        offspring.history.push(format!("Bred at {}", ts));
        parent_1.history.clear();
        parent_1.history.push(format!("Used in breeding at {}", ts));
        parent_2.history.clear();
        parent_2.history.push(format!("Used in breeding at {}", ts));

        msg!(
            "Bred NFT '{}' level={} xp={} score={}",
            offspring.name,
            offspring.level,
            offspring.xp,
            offspring.score
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BreedNfts<'info> {
    /// 親 NFT その１
    #[account(mut)]
    pub parent_1: Account<'info, GameNft>,
    /// 親 NFT その２
    #[account(mut)]
    pub parent_2: Account<'info, GameNft>,
    /// 生成される子 NFT
    #[account(init, payer = breeder, space = 8 + GameNft::SIZE)]
    pub offspring: Account<'info, GameNft>,
    /// ブリーダー（署名者）
    #[account(signer)]
    pub breeder: Signer<'info>,
    /// 時刻取得用 Sysvar
    pub clock: Sysvar<'info, Clock>,
    /// システムプログラム
    pub system_program: Program<'info, System>,
    /// 初期資金用
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[account]
pub struct GameNft {
    pub owner:    Pubkey,
    pub name:     String,
    pub level:    u8,
    pub xp:       u64,
    pub score:    u64,
    pub trait_id: u32,
    pub mixed:    u16,
    pub history:  Vec<String>,
}

impl GameNft {
    // 文字列や Vec の最大長を考慮したサイズ
    pub const SIZE: usize = 32
        + 4 + 64   // name max 64 bytes
        + 1        // level
        + 8        // xp
        + 8        // score
        + 4        // trait_id
        + 2        // mixed
        + 4 + 256; // history vector max 256 bytes
}

#[error]
pub enum ErrorCode {
    #[msg("Mutable accounts must be different.")]
    DuplicateMutableAccount,
}
