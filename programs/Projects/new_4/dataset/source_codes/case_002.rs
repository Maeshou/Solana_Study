use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222");

#[program]
pub mod reinit_vault {
    use super::*;

    pub fn setup_vault(
        ctx: Context<SetupVault>,
        owner: Pubkey,
        balance: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = owner;
        vault.balance = balance;
        vault.open = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultData {
    pub owner: Pubkey,
    pub balance: u64,
    pub open: bool,
}
