use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqUpdateLst");

#[program]
pub mod nft_listing_update {
    use super::*;

    pub fn update_listing(
        ctx: Context<UpdateListing>,
        new_price: u64,
        new_duration: u64,
    ) -> Result<()> {
        let acct_info = &mut ctx.accounts.listing_account.to_account_info();
        let data = &mut acct_info.data.borrow_mut();

        // data[15] が存在しなければエラー (最低 16 バイト必要)
        if data.get(15).is_none() {
            return err!(ErrorCode::DataTooShort);
        }

        let price_le = new_price.to_le_bytes();
        let dur_le   = new_duration.to_le_bytes();

        // 0..7 → price_le、8..15 → dur_le をバイト毎に代入
        data[0]  = price_le[0];
        data[1]  = price_le[1];
        data[2]  = price_le[2];
        data[3]  = price_le[3];
        data[4]  = price_le[4];
        data[5]  = price_le[5];
        data[6]  = price_le[6];
        data[7]  = price_le[7];

        data[8]  = dur_le[0];
        data[9]  = dur_le[1];
        data[10] = dur_le[2];
        data[11] = dur_le[3];
        data[12] = dur_le[4];
        data[13] = dur_le[5];
        data[14] = dur_le[6];
        data[15] = dur_le[7];

        msg!(
            "Listing {} updated: price={} lamports, duration={}s by {}",
            acct_info.key(),
            new_price,
            new_duration,
            ctx.accounts.updater.key(),
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateListing<'info> {
    /// CHECK: owner チェック省略の危険な AccountInfo
    #[account(mut)]
    pub listing_account: AccountInfo<'info>,

    /// 署名者であることのみ検証
    pub updater: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("アカウントデータが想定より短いです")]
    DataTooShort,
}
