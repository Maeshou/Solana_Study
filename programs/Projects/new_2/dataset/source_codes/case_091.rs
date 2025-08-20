use anchor_lang::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use anchor_lang::solana_program::sysvar::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqWorkshopSaleV2");

/// Borsh でシリアライズする販売データ構造体
#[derive(BorshSerialize, BorshDeserialize)]
pub struct WorkshopSale {
    pub item_id: u32,      // 成果物の識別子
    pub price:   u64,      // 販売価格 (lamports)
    pub author:  Pubkey,   // 作者
    pub listed_at: i64,    // UNIXタイムスタンプ
}

#[program]
pub mod nft_workshop_sale_v2 {
    use super::*;

    /// ワークショップで作成したアイテムを販売する  
    /// (`sale_account` の owner チェックを一切行っていないため、  
    ///  攻撃者が他人の成果物アカウントを指定して無断で販売できます)
    pub fn list_workshop_item(
        ctx: Context<ListWorkshopItem>,
        item_id: u32,
        price:   u64,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let sale = WorkshopSale {
            item_id,
            price,
            author: ctx.accounts.author.key(),
            listed_at: now,
        };

        let mut buf = ctx.accounts.sale_account.data.borrow_mut();
        // Borsh で一度に serialize
        sale.serialize(&mut &mut buf[..])
            .map_err(|_| ErrorCode::SerializationFailed)?;

        msg!(
            "Workshop item {} listed at {} lamports by {} at {}",
            item_id,
            price,
            sale.author,
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListWorkshopItem<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub sale_account: AccountInfo<'info>,

    /// 作者の署名のみ検証
    pub author: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データのシリアライズに失敗しました")]
    SerializationFailed,
}
