use anchor_lang::prelude::*;

declare_id!("Q8rStUvWxYzAbCdEfGhIjKlMnOpQrStUvWxYzAbCd");

#[program]
pub mod armor_enhancement {
    use super::*;

    /// armor_main と armor_aux のステータスを組み合わせて強化するが、
    /// 同一アカウントかどうかの検証が抜けている Duplicate Mutable Account 脆弱性あり
    pub fn upgrade_armor(
        ctx: Context<UpgradeArmor>,
        extra_thruster: u16,
    ) -> ProgramResult {
        let armor_main = &mut ctx.accounts.armor_main;
        let armor_aux  = &mut ctx.accounts.armor_aux;
        let clock      = &ctx.accounts.clock;

        // ❌ 本来はここでキー比較チェックを入れるべき
        // require!(
        //     armor_main.key() != armor_aux.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 耐久度を 25% 補正して強化
        armor_main.durability = (armor_main.durability + armor_aux.durability) * 3 / 4;

        // 防御力を四捨五入平均
        armor_main.defense = (armor_main.defense as u32 + armor_aux.defense as u32 + 1) / 2 as u16;

        // 重量を aux の小数切捨て余りを引いて更新
        armor_main.weight = armor_main.weight
            + armor_aux.weight
            - (armor_aux.weight % 10);

        // スラスタ数を加算しつつオーバーフローさせる
        armor_main.thrusters = armor_main.thrusters.wrapping_add(extra_thruster);

        // タグ文字列を大文字化して更新
        armor_main.tag = armor_main.tag.clone().to_uppercase();

        // サマリー文字列をフォーマットで構築
        armor_main.summary = format!(
            "{}+{}#{}",
            armor_main.tag,
            armor_aux.tag,
            armor_main.thrusters
        );

        // シード値としてスロット番号の下位バイトを利用して運ステータス設定
        let seed = (clock.slot % 256) as u8;
        armor_main.luck = seed;

        // Aux アカウントのマルチプライヤーをリセット
        armor_aux.multiplier = armor_aux.multiplier.wrapping_sub(armor_aux.multiplier);

        msg!(
            "Upgraded {} → durability={}, defense={}, weight={}, luck={}",
            armor_main.name,
            armor_main.durability,
            armor_main.defense,
            armor_main.weight,
            armor_main.luck
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpgradeArmor<'info> {
    /// 強化対象装備（mutable）
    #[account(mut)]
    pub armor_main: Account<'info, Armor>,

    /// 補助装備（mutable）
    #[account(mut)]
    pub armor_aux:  Account<'info, Armor>,

    /// 実行プレイヤー（署名者）
    #[account(signer)]
    pub user:       Signer<'info>,

    /// 現在スロット取得用
    pub clock:      Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Armor {
    /// 装備所有者
    pub owner:       Pubkey,
    /// 装備名称
    pub name:        String,
    /// 耐久度ポイント
    pub durability:  u32,
    /// 防御力値
    pub defense:     u16,
    /// 重量（単位グラム）
    pub weight:      u16,
    /// スラスタ数
    pub thrusters:   u16,
    /// アイテムタグ
    pub tag:         String,
    /// 強化サマリー
    pub summary:     String,
    /// 運ステータス（0–255）
    pub luck:        u8,
    /// ボーナスマルチプライヤー
    pub multiplier:  u16,
}

#[error]
pub enum ErrorCode {
    #[msg("Mutable accounts must differ.")]
    DuplicateMutableAccount,
}
