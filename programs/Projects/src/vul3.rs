#[program]
pub mod insecure_vault {
    use super::*;

    // 権限の更新（脆弱性あり）
    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        // Vulnerability: Authority is updated without verifying if the current authority signed the transaction
        ctx.accounts.vault.authority = ctx.accounts.new_authority.key();
        Ok(())
    }

    // 資金の引き出し（脆弱性あり）
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        // Vulnerability: No signature verification, allowing unauthorized access
        let vault = &mut ctx.accounts.vault;
        let recipient = &ctx.accounts.recipient;
        let system_program = &ctx.accounts.system_program;

        // 引き出しロジック
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **recipient.to_account_info().try_borrow_mut_lamports()? += amount;

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
    pub authority: AccountInfo<'info>, // No signature verification
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub authority: AccountInfo<'info>, // No signature verification
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub lamports: u64, // 保有している資金
}
