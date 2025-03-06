use anchor_lang::prelude::*;

declare_id!("FhBr4Xe9pythYg4Nn3gWYhZyygQxU1xCe9fMMzp7nbZB");

#[program]
pub mod insecure_update {
    use super::*;
    
    pub fn initialize_vault(ctx: Context<InitializeVault>, authority: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = authority;
        Ok(())
    }

    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        let accs = ctx.accounts;
        accs.vault.authority = accs.new_authority.key();
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = user, space = 8 + 32)] // 8バイトはディスクリミネータ、32バイトはPubkey
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(mut,has_one= authority)]
    pub vault: Account<'info, Vault>,
    pub new_authority: AccountInfo<'info>,
    ///CHECK: 実験のため、このフィールドに関する安全性のチェックは行いません。
    pub authority: UncheckedAccount<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
}
