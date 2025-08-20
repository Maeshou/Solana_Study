use anchor_lang::prelude::*;

declare_id!("MixChk8888888888888888888888888888888888");

#[program]
pub mod mixed_check8 {
    pub fn post_review(ctx: Context<Post>, text: String) -> Result<()> {
        // item.owner は検証あり
        require_keys_eq!(ctx.accounts.item.owner, ctx.accounts.user.key(), CustomError::NotAllowed);
        ctx.accounts.item.last_review = text;
        // notif_buf は検証なし
        let _ = ctx.accounts.notif_buf.data.borrow();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Post<'info> {
    #[account(mut, has_one = owner)]
    pub item: Account<'info, ItemData>,
    pub owner: Signer<'info>,

    /// CHECK: 通知バッファ未検証
    #[account(mut)]
    pub notif_buf: AccountInfo<'info>,
}

#[account]
pub struct ItemData {
    pub owner: Pubkey,
    pub last_review: String,
}
