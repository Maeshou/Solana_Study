use anchor_lang::prelude::*;

declare_id!("MixChkB122222222222222222222222222222222");

#[program]
pub mod mixed_check12 {
    pub fn list_item(ctx: Context<ListItem>, price: u64) -> Result<()> {
        // item.seller と署名者チェックあり
        require_keys_eq!(
            ctx.accounts.item.seller,
            ctx.accounts.seller.key(),
            CustomError::NotSeller
        );
        ctx.accounts.item.price = price;
        // escrow_acc は所有者チェックなし
        let _ = ctx.accounts.escrow_acc.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ListItem<'info> {
    #[account(mut, has_one = seller)]
    pub item: Account<'info, Listing>,
    pub seller: Signer<'info>,
    #[account(mut)]
    pub escrow_acc: AccountInfo<'info>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub price: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Not seller")]
    NotSeller,
}
