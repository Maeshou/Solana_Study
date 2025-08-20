
use anchor_lang::prelude::*;

declare_id!("TokenMeta1111111111111111111111111111111111");

#[program]
pub mod case6 {
    use super::*;

    pub fn update_metadata(ctx: Context<UpdateMetadata>, uri: String) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;
        metadata.uri = uri.clone();
        metadata.edit_log.push(uri);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub metadata: Account<'info, TokenMetadata>,
    #[account(signer)]
    pub updater: AccountInfo<'info>,
}

#[account]
pub struct TokenMetadata {
    pub uri: String,
    pub creator: Pubkey,
    pub edit_log: Vec<String>,
}
