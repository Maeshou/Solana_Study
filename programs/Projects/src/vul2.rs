#[program]
pub mod insecure_update {
    use super::*;

    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        // Vulnerability: The code changes the vault authority without checking if the authority signed the message
        ctx.accounts.vault.authority = ctx.accounts.new_authority.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    pub new_authority: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

#[account]
pub struct Vault {
    pub token_account: Pubkey,
    pub authority: Pubkey,
}
