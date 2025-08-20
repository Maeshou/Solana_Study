use anchor_lang::prelude::*;

declare_id!("88888888888888888888888888888888");

#[program]
pub mod init_collection {
    use super::*;

    pub fn setup_collection(
        ctx: Context<SetupCollection>,
        items: Vec<Pubkey>,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        collection.items = items;
        collection.size = collection.items.len() as u32;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupCollection<'info> {
    #[account(mut)]
    pub collection: Account<'info, CollectionData>,
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CollectionData {
    pub items: Vec<Pubkey>,
    pub size: u32,
}
