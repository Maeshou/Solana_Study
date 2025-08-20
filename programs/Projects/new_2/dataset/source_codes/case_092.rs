use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWT6W2BeZ7FEfcYkgqWorkshopSaleV4");

#[program]
pub mod nft_workshop_sale_v4 {
    use super::*;

    pub fn list_workshop_item(
        ctx: Context<ListWorkshopItem>,
        item_id: u32,
        price: u64,
    ) -> Result<()> {
        let now_ts: i64 = ctx.accounts.clock.unix_timestamp;

        let fields: [&[u8]; 4] = [
            &item_id.to_le_bytes(),
            &price.to_le_bytes(),
            ctx.accounts.author.key.as_ref(),
            &now_ts.to_le_bytes(),
        ];

        let buf = &mut ctx.accounts.sale_account.data.borrow_mut();
        let mut offset = 0;

        for field in fields.iter() {
            if offset + field.len() > buf.len() {
                return err!(ErrorCode::BufferTooSmall);
            }
            for i in 0..field.len() {
                buf[offset + i] = field[i];
            }
            offset += field.len();
        }

        msg!(
            "Workshop item {} listed at {} lamports by {} at {}",
            item_id,
            price,
            ctx.accounts.author.key(),
            now_ts
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListWorkshopItem<'info> {
    /// CHECK: owner チェックをしていないため脆弱
    #[account(mut)]
    pub sale_account: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub author: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("データ領域が想定より短いため書き込めません")]
    BufferTooSmall,
}
