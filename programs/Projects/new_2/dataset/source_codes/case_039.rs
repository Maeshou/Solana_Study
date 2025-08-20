use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqLevelUp1");

#[program]
pub mod nft_level_up {
    use super::*;

    /// NFTメタデータの先頭2バイトを「レベル」として扱い、1だけインクリメントする  
    /// （owner == program_id のチェックを一切行っていないため、任意のアカウントを操作可能）
    pub fn level_up(ctx: Context<LevelUp>) -> Result<()> {
        let acct = &mut ctx.accounts.nft_meta.to_account_info();
        let data = &mut acct.data.borrow_mut();

        // レベル用に先頭2バイトが必要
        if data.len() < 2 {
            return err!(ErrorCode::InsufficientData);
        }

        // イテレータで先頭2バイトを取り出し、レベル読み書き
        let mut iter = data.iter_mut();
        let lo_byte = iter.next().unwrap();
        let hi_byte = iter.next().unwrap();

        // 現在のレベルをリトルエンディアンで取得し、+1
        let mut level = u16::from_le_bytes([*lo_byte, *hi_byte]);
        level = level.saturating_add(1);

        // 新しいレベルを書き戻し
        let [new_lo, new_hi] = level.to_le_bytes();
        *lo_byte = new_lo;
        *hi_byte = new_hi;

        msg!("Leveled up NFT {} → new level {}", acct.key(), level);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LevelUp<'info> {
    /// CHECK: ownerフィールドの確認を行っていない生のAccountInfo
    #[account(mut)]
    pub nft_meta: AccountInfo<'info>,

    /// 呼び出し元が署名者であることのみ検証
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ領域にレベル用の2バイトがありません")]
    InsufficientData,
}
