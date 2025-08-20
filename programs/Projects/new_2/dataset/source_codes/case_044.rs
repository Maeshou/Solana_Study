use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqDynLst01");

#[program]
pub mod dynamic_discount_listing {
    use super::*;

    /// ベース価格と割引率から最終価格を計算し、
    /// listing_account に連続バイトで書き込む。
    /// （owner チェックを行っていないため、攻撃者が他人の
    ///  アカウントを指定して無断で出品できます）
    pub fn list_with_discount(
        ctx: Context<ListWithDiscount>,
        base_price: u64,
        discount_pct: u8,
    ) -> Result<()> {
        let acct = &mut ctx.accounts.listing_account.to_account_info();
        let mut data = acct.data.borrow_mut();

        // 割引後価格を計算
        let final_price = base_price
            .saturating_mul((100u64 - discount_pct as u64))
            / 100u64;
        // 現在の UNIX タイムスタンプを取得
        let ts = Clock::get()?.unix_timestamp as u64;

        // 書き込むバイト列をシーケンシャルに組み立て
        // [ts:8][base_price:8][final_price:8][discount_pct:1][seller_pubkey:32]
        let mut seq = Vec::with_capacity(8 + 8 + 8 + 1 + 32);
        seq.extend_from_slice(&ts.to_le_bytes());
        seq.extend_from_slice(&base_price.to_le_bytes());
        seq.extend_from_slice(&final_price.to_le_bytes());
        seq.push(discount_pct);
        seq.extend_from_slice(&ctx.accounts.seller.key().to_bytes());

        // データ領域が足りない場合はエラー
        if data.len() < seq.len() {
            return err!(ErrorCode::DataTooShort);
        }

        // イテレータ + next() で連続的に書き込み
        let mut it = data.iter_mut();
        for &b in &seq {
            *it.next().unwrap() = b;
        }

        msg!(
            "NFT {} listed by {}: base={} → final={} (discount={}%)",
            acct.key(),
            ctx.accounts.seller.key(),
            base_price,
            final_price,
            discount_pct
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListWithDiscount<'info> {
    /// CHECK: owner == program_id の検証を一切行っていない AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 出品者の署名のみ検証
    pub seller: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータ領域が不足しています")]
    DataTooShort,
}
