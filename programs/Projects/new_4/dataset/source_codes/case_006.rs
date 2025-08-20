use anchor_lang::prelude::*;

declare_id!("66666666666666666666666666666666");

#[program]
pub mod init_metadata {
    use super::*;

    pub fn load_metadata(
        ctx: Context<LoadMetadata>,
        url: String,
        version: u8,
    ) -> Result<()> {
        let meta = &mut ctx.accounts.metadata;
        meta.url = url;
        meta.version = version;
        meta.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LoadMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, MetadataData>,
    pub loader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MetadataData {
    pub url: String,
    pub version: u8,
    pub updated_at: i64,
}
