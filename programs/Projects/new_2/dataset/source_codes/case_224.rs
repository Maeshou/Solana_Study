use anchor_lang::prelude::*;

declare_id!("VulnEx22000000000000000000000000000000000022");

#[program]
pub mod vault_manager {
    pub fn sweep_all(ctx: Context<Ctx2>) -> Result<()> {
        // vault_log は未検証
        **ctx.accounts.vault_log.lamports.borrow_mut() += **ctx.accounts.vault.lamports.borrow();
        **ctx.accounts.vault.lamports.borrow_mut() = 0;
        // vault は has_one で authority 検証済み
        msg!("Vault emptied by authority");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx2<'info> {
    #[account(mut)]
    pub vault_log: AccountInfo<'info>,
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, VaultData>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
