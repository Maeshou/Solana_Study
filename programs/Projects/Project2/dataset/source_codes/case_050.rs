use anchor_lang::prelude::*;

declare_id!("BufUpdate10101010101010101010101010101010");

#[program]
pub mod buffer_updater {
    use super::*;

    pub fn change(ctx: Context<Change>, data: u64) -> Result<()> {
        let mut buf = ctx.accounts.acc.data.borrow_mut();
        buf[..8].copy_from_slice(&data.to_le_bytes());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Change<'info> {
    #[account(mut, seeds = [b"item", author.key().as_ref()], bump)]
    pub acc: AccountInfo<'info>,
    pub author: Signer<'info>,
}
