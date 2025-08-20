use anchor_lang::prelude::*;

declare_id!("D4pQrStUvWxYzAbCdEfGhIjKlMnOpQrStUvWxYzAb");

#[program]
pub mod item_merger {
    use super::*;

    /// 二つのゲームアイテムをマージするが、
    /// 同一アカウントかどうかのチェックをしていない！
    pub fn merge_items(ctx: Context<MergeItems>) -> ProgramResult {
        let item_x = &mut ctx.accounts.item_x;
        let item_y = &mut ctx.accounts.item_y;

        // ❌ 本来は以下のチェックが必要
        // require!(
        //     item_x.key() != item_y.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 攻撃力を合算（u64飽和加算）
        item_x.power = item_x.power.saturating_add(item_y.power);

        // レア度を合算（u8飽和加算）
        item_x.rarity = item_x.rarity.saturating_add(item_y.rarity);

        // 属性フラグをビット OR
        item_x.flags = item_x.flags | item_y.flags;

        // 名前を連結
        item_x.name.push_str("-merged-");
        item_x.name.push_str(&item_y.name);

        msg!(
            "Player {} merged '{}' → '{}' (power={}, rarity={}, flags={})",
            ctx.accounts.player.key(),
            item_x.name,
            item_y.name,
            item_x.power,
            item_x.rarity,
            item_x.flags
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MergeItems<'info> {
    /// マージ元アイテム
    #[account(mut)]
    pub item_x: Account<'info, GameItem>,

    /// マージ先アイテム
    #[account(mut)]
    pub item_y: Account<'info, GameItem>,

    /// 実行プレイヤー
    #[account(signer)]
    pub player: Signer<'info>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameItem {
    /// アイテム所有者
    pub owner:  Pubkey,
    /// アイテム名
    pub name:   String,
    /// 攻撃力
    pub power:  u64,
    /// レア度
    pub rarity: u8,
    /// 属性フラグ
    pub flags:  u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Duplicate mutable account detected.")]
    DuplicateMutableAccount,
}
