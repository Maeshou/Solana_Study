
// =============================================================================
// 2. Secure Data Storage with Program Ownership
// =============================================================================
#[program]
pub mod secure_data_storage {
    use super::*;

    pub fn initialize_storage(ctx: Context<InitializeStorage>, data: String) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        storage_account.owner = ctx.accounts.authority.key();
        storage_account.data = data;
        storage_account.bump = *ctx.bumps.get("storage_account").unwrap();
        Ok(())
    }

    pub fn update_data(ctx: Context<UpdateData>, new_data: String) -> Result<()> {
        let storage_account = &mut ctx.accounts.storage_account;
        storage_account.data = new_data;
        Ok(())
    }
}

#[account]
pub struct StorageAccount {
    pub owner: Pubkey,
    pub data: String,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeStorage<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 200 + 1,
        seeds = [b"storage", authority.key().as_ref()],
        bump
    )]
    pub storage_account: Account<'info, StorageAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(
        mut,
        seeds = [b"storage", authority.key().as_ref()],
        bump = storage_account.bump,
        constraint = storage_account.owner == authority.key()
    )]
    pub storage_account: Account<'info, StorageAccount>,
    
    pub authority: Signer<'info>,
}
