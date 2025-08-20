use anchor_lang::prelude::*;

declare_id!("VaultVuln111111111111111111111111111111111");

#[program]
pub mod vault_vuln {
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // vault.authority の検証なし
        let vault_acc = ctx.accounts.vault.to_account_info();
        let dest_acc  = ctx.accounts.destination.to_account_info();
        **dest_acc.lamports.borrow_mut() += amount;
        **vault_acc.lamports.borrow_mut() -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: authority 未検証で誰でも呼べる
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
