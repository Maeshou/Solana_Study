#[program]
pub mod authorized_program {
    use super::*;

    pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let recipient = &mut ctx.accounts.recipient;

        // 本来ならSignerの確認が必要
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **recipient.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(
        mut,
        has_one = delegate_program
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    /// CHECK: Vaultからの権限チェックは脆弱性のため不十分
    pub delegate_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
