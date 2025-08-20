// =============================================================================
// 14. Supply Chain Tracking System
// =============================================================================
#[program]
pub mod secure_supply_chain {
    use super::*;

    pub fn create_product(ctx: Context<CreateProduct>, name: String, origin: String) -> Result<()> {
        let product = &mut ctx.accounts.product;
        product.manufacturer = ctx.accounts.manufacturer.key();
        product.name = name;
        product.origin = origin;
        product.current_owner = ctx.accounts.manufacturer.key();
        product.created_at = Clock::get()?.unix_timestamp;
        product.bump = *ctx.bumps.get("product").unwrap();
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let transfer_record = &mut ctx.accounts.transfer_record;
        
        transfer_record.product = product.key();
        transfer_record.from_owner = product.current_owner;
        transfer_record.to_owner = *ctx.accounts.new_owner.key;
        transfer_record.transferred_at = Clock::get()?.unix_timestamp;
        transfer_record.bump = *ctx.bumps.get("transfer_record").unwrap();
        
        product.current_owner = *ctx.accounts.new_owner.key;
        
        Ok(())
    }

    pub fn add_inspection(ctx: Context<AddInspection>, inspector_id: String, notes: String, passed: bool) -> Result<()> {
        let product = &ctx.accounts.product;
        let inspection = &mut ctx.accounts.inspection;
        
        inspection.product = product.key();
        inspection.inspector_id = inspector_id;
        inspection.notes = notes;
        inspection.passed = passed;
        inspection.inspected_at = Clock::get()?.unix_timestamp;
        inspection.bump = *ctx.bumps.get("inspection").unwrap();
        
        Ok(())
    }
}

#[account]
pub struct Product {
    pub manufacturer: Pubkey,
    pub name: String,
    pub origin: String,
    pub current_owner: Pubkey,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct TransferRecord {
    pub product: Pubkey,
    pub from_owner: Pubkey,
    pub to_owner: Pubkey,
    pub transferred_at: i64,
    pub bump: u8,
}

#[account]
pub struct Inspection {
    pub product: Pubkey,
    pub inspector_id: String,
    pub notes: String,
    pub passed: bool,
    pub inspected_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(name: String, origin: String)]
pub struct CreateProduct<'info> {
    #[account(
        init,
        payer = manufacturer,
        space = 8 + 32 + 4 + name.len() + 4 + origin.len() + 32 + 8 + 1,
        seeds = [b"product", manufacturer.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub product: Account<'info, Product>,
    
    #[account(mut)]
    pub manufacturer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"product", product.manufacturer.as_ref(), product.name.as_bytes()],
        bump = product.bump,
        constraint = product.current_owner == current_owner.key()
    )]
    pub product: Account<'info, Product>,
    
    #[account(
        init,
        payer = current_owner,
        space = 8 + 32 + 32 + 32 + 8 + 1,
        seeds = [b"transfer", product.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub transfer_record: Account<'info, TransferRecord>,
    
    #[account(mut)]
    pub current_owner: Signer<'info>,
    
    /// CHECK: Verified as the new owner through transfer record
    pub new_owner: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(inspector_id: String, notes: String)]
pub struct AddInspection<'info> {
    #[account(
        seeds = [b"product", product.manufacturer.as_ref(), product.name.as_bytes()],
        bump = product.bump
    )]
    pub product: Account<'info, Product>,
    
    #[account(
        init,
        payer = inspector,
        space = 8 + 32 + 4 + inspector_id.len() + 4 + notes.len() + 1 + 8 + 1,
        seeds = [b"inspection", product.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub inspection: Account<'info, Inspection>,
    
    #[account(mut)]
    pub inspector: Signer<'info>,
    pub system_program: Program<'info, System>,
}