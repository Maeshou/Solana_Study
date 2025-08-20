use anchor_lang::prelude::*;

declare_id!("VulnEx68000000000000000000000000000000000068");

#[program]
pub mod example68 {
    pub fn purge_metadata(ctx: Context<Ctx68>) -> Result<()> {
        // meta_buf is unchecked
        ctx.accounts.meta_buf.data.borrow_mut().fill(0);
        // metadata is has_one = updater
        let md = &mut ctx.accounts.metadata;
        md.entries.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx68<'info> {
    #[account(mut)]
    pub meta_buf: AccountInfo<'info>,
    #[account(mut, has_one = updater)]
    pub metadata: Account<'info, Metadata>,
    pub updater: Signer<'info>,
}

#[account]
pub struct Metadata {
    pub updater: Pubkey,
    pub entries: Vec<String>,
}
