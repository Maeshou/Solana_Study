use anchor_lang::prelude::*;

declare_id!("T5uVwXyZaBcDeFgHiJkLmNoPqRsTuVwXyZaBcDeFg");

#[program]
pub mod nft_market_sale {
    use super::*;

    /// seller_acc と buyer_acc で NFT を売買するが、
    /// 同一アカウントチェックが抜けている Duplicate Mutable Account 脆弱性あり
    pub fn execute_sale(
        ctx: Context<ExecuteSale>,
        sale_price: u64,
        commission_rate: u8,
        tag_suffix: String,
    ) -> ProgramResult {
        let seller = &mut ctx.accounts.seller_acc;
        let buyer  = &mut ctx.accounts.buyer_acc;
        let nft    = &mut ctx.accounts.nft;
        let now    = ctx.accounts.clock.unix_timestamp;

        // ❌ 本来はここでキー比較チェックを入れるべき
        // require!(
        //     seller.key() != buyer.key(),
        //     ErrorCode::DuplicateMutableAccount
        // );

        // 売却代金を seller に加算、buyer から減算
        seller.balance       = seller.balance + sale_price;
        buyer.balance        = buyer.balance - sale_price;

        // 手数料を計算し seller から控除
        let commission       = sale_price * (commission_rate as u64) / 100;
        seller.balance       = seller.balance - commission;

        // NFT の所有者変更
        nft.owner            = buyer.owner;

        // 売買回数の更新
        seller.sale_count    = seller.sale_count + 1;
        buyer.purchase_count = buyer.purchase_count + 1;

        // ノートに売買履歴をタグ付け
        seller.note          = seller.note.clone() + "|sold@" + &now.to_string();
        buyer.note           = buyer.note.clone()  + "|bought@" + &now.to_string();

        // NFT タグをサフィックス付きで大文字化
        nft.tag               = (nft.tag.clone() + "-" + &tag_suffix).to_uppercase();

        msg!(
            "Sale executed: {} sold NFT to {} for {} lamports (commission={}, time={})",
            seller.owner,
            buyer.owner,
            sale_price,
            commission,
            now
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteSale<'info> {
    /// 売り手アカウント（mutable）
    #[account(mut)]
    pub seller_acc: Account<'info, UserProfile>,

    /// 買い手アカウント（mutable）
    #[account(mut)]
    pub buyer_acc:  Account<'info, UserProfile>,

    /// 売買対象の NFT
    #[account(mut)]
    pub nft:        Account<'info, GameNft>,

    /// オペレーション担当者（署名者）
    #[account(signer)]
    pub operator:   Signer<'info>,

    /// 時刻取得用 Sysvar
    pub clock:      Sysvar<'info, Clock>,

    /// システムプログラム
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    /// アカウント所有者
    pub owner:           Pubkey,
    /// 残高
    pub balance:         u64,
    /// 売却回数
    pub sale_count:      u32,
    /// 購入回数
    pub purchase_count:  u32,
    /// 自由メモ欄
    pub note:            String,
}

#[account]
pub struct GameNft {
    /// NFT 所有者
    pub owner:           Pubkey,
    /// NFT タグ
    pub tag:             String,
}

#[error]
pub enum ErrorCode {
    #[msg("Mutable accounts must differ.")]
    DuplicateMutableAccount,
}
