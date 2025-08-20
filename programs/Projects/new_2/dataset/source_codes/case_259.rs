use anchor_lang::prelude::*;

declare_id!("VulnEx73000000000000000000000000000000000073");

#[program]
pub mod example73 {
    pub fn uppercase_message(ctx: Context<Ctx73>) -> Result<()> {
        // msg_acc: OWNER CHECK SKIPPED, 生文字列を累積
        let raw = ctx.accounts.msg_acc.data.borrow_mut();
        let s = String::from_utf8(raw.clone()).unwrap_or_default().to_uppercase();
        *raw = s.into_bytes();

        // message_box: has_one = author
        ctx.accounts.message_box.history.push(raw.clone());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx73<'info> {
    #[account(mut, has_one = author)]
    pub message_box: Account<'info, MessageBox>,
    pub author: Signer<'info>,
    #[account(mut)]
    pub msg_acc: AccountInfo<'info>,  // unchecked
}

#[account]
pub struct MessageBox {
    pub author: Pubkey,
    pub history: Vec<Vec<u8>>,
}
