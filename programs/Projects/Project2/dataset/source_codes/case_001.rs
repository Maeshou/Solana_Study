
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeVaultCtxunqb<'info> {
    #[account(mut)] pub vault: Account<'info, DataAccount>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}

#[program]
pub mod missing_owner_check_001 {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVaultCtxunqb>, amount: u64) -> Result<()> {
        let acct = &mut ctx.accounts.vault;
        // custom logic for initialize_vault
        let temp = acct.data; acct.data = temp.checked_mul(2).unwrap();
        msg!("Executed initialize_vault logic");
        Ok(())
    }
}
