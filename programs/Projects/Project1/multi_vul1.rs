#[program]
pub mod vault_manager {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, authority: Pubkey) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = authority;
        Ok(())
    }

    pub fn delegate_withdraw(ctx: Context<DelegateWithdraw>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // `authorized_program`に実行を委任
        vault.delegate_program = ctx.accounts.delegate_program.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DelegateWithdraw<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    //識別子が必要
    pub authority: Signer<'info>,
    /// CHECK: 委任プログラムは安全性を保証しない
    pub delegate_program: UncheckedAccount<'info>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub delegate_program: Pubkey,
}
