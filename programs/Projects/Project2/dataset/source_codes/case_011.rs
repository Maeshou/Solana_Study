// =============================================================================
// 11. Real Estate Property Management System
// =============================================================================
use anchor_lang::prelude::*;

#[program]
pub mod secure_real_estate {
    use super::*;

    pub fn register_property(ctx: Context<RegisterProperty>, address: String, price: u64, property_type: PropertyType) -> Result<()> {
        let property = &mut ctx.accounts.property;
        property.owner = ctx.accounts.owner.key();
        property.address = address;
        property.price = price;
        property.property_type = property_type;
        property.is_listed = true;
        property.tenant = None;
        property.bump = *ctx.bumps.get("property").unwrap();
        Ok(())
    }

    pub fn lease_property(ctx: Context<LeaseProperty>, lease_duration: i64) -> Result<()> {
        let property = &mut ctx.accounts.property;
        let lease = &mut ctx.accounts.lease;
        
        require!(property.is_listed, RealEstateError::PropertyNotListed);
        
        lease.property = property.key();
        lease.tenant = ctx.accounts.tenant.key();
        lease.landlord = property.owner;
        lease.start_date = Clock::get()?.unix_timestamp;
        lease.end_date = lease.start_date + lease_duration;
        lease.monthly_rent = property.price / 12;
        lease.is_active = true;
        lease.bump = *ctx.bumps.get("lease").unwrap();
        
        property.tenant = Some(ctx.accounts.tenant.key());
        property.is_listed = false;
        
        Ok(())
    }

    pub fn pay_rent(ctx: Context<PayRent>) -> Result<()> {
        let lease = &ctx.accounts.lease;
        
        require!(lease.is_active, RealEstateError::LeaseNotActive);
        require!(Clock::get()?.unix_timestamp <= lease.end_date, RealEstateError::LeaseExpired);
        
        // Transfer rent payment
        **ctx.accounts.tenant.lamports.borrow_mut() -= lease.monthly_rent;
        **ctx.accounts.landlord.lamports.borrow_mut() += lease.monthly_rent;
        
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PropertyType {
    Residential,
    Commercial,
    Industrial,
}

#[account]
pub struct Property {
    pub owner: Pubkey,
    pub address: String,
    pub price: u64,
    pub property_type: PropertyType,
    pub is_listed: bool,
    pub tenant: Option<Pubkey>,
    pub bump: u8,
}

#[account]
pub struct Lease {
    pub property: Pubkey,
    pub tenant: Pubkey,
    pub landlord: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub monthly_rent: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(address: String)]
pub struct RegisterProperty<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 4 + address.len() + 8 + 32 + 1 + 33 + 1,
        seeds = [b"property", owner.key().as_ref(), address.as_bytes()],
        bump
    )]
    pub property: Account<'info, Property>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LeaseProperty<'info> {
    #[account(
        mut,
        seeds = [b"property", property.owner.as_ref(), property.address.as_bytes()],
        bump = property.bump,
        constraint = property.owner != tenant.key() @ RealEstateError::OwnerCannotLease
    )]
    pub property: Account<'info, Property>,
    
    #[account(
        init,
        payer = tenant,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1,
        seeds = [b"lease", property.key().as_ref(), tenant.key().as_ref()],
        bump
    )]
    pub lease: Account<'info, Lease>,
    
    #[account(mut)]
    pub tenant: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PayRent<'info> {
    #[account(
        seeds = [b"lease", lease.property.as_ref(), tenant.key().as_ref()],
        bump = lease.bump,
        constraint = lease.tenant == tenant.key()
    )]
    pub lease: Account<'info, Lease>,
    
    #[account(mut)]
    pub tenant: Signer<'info>,
    
    /// CHECK: Verified through lease landlord field
    #[account(
        mut,
        constraint = landlord.key() == lease.landlord
    )]
    pub landlord: AccountInfo<'info>,
}

#[error_code]
pub enum RealEstateError {
    #[msg("Property is not listed")]
    PropertyNotListed,
    #[msg("Lease is not active")]
    LeaseNotActive,
    #[msg("Lease has expired")]
    LeaseExpired,
    #[msg("Owner cannot lease their own property")]
    OwnerCannotLease,
}
