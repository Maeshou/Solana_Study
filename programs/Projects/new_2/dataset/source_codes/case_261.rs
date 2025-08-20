use anchor_lang::prelude::*;
use data_encoding::BASE64;

declare_id!("VulnEx75000000000000000000000000000000000075");

#[program]
pub mod example75 {
    pub fn base64_encode(ctx: Context<Ctx75>) -> Result<()> {
        // src_acc: OWNER CHECK SKIPPED
        let raw = ctx.accounts.src_acc.data.borrow().clone();
        let enc = BASE64.encode(&raw);
        // dst_acc: has_one = updater
        let mut dst = ctx.accounts.dst_acc.data.borrow_mut();
        dst.clear();
        dst.extend_from_slice(enc.as_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx75<'info> {
    #[account(mut)]
    pub src_acc: AccountInfo<'info>,   // unchecked
    #[account(mut, has_one = updater)]
    pub dst_acc: Account<'info, DataBuffer>,
    pub updater: Signer<'info>,
}

#[account]
pub struct DataBuffer {
    pub updater: Pubkey,
    pub data: Vec<u8>,
}
