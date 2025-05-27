#[program]
pub mod secure_vault {
    use super::*;

    // 安全な権限の更新
    pub fn update_authority(ctx: Context<UpdateAuthority>) -> Result<()> {
        // Ensure only the current authority can update the authority
        ctx.accounts.vault.authority = ctx.accounts.new_authority.key();
        Ok(())
    }

    // 安全な資金の引き出し
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let recipient = &ctx.accounts.recipient;

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
    pub new_authority: Signer<'info>, // 新しい権限を設定するアカウント
    #[account(signer)]
    pub authority: Signer<'info>, // 現在の権限アカウント (署名を要求)
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>, // 資金を受け取るアカウント
    #[account(signer)]
    pub authority: Signer<'info>, // 権限アカウント (署名を要求)
    pub system_program: Program<'info, System>, // システムプログラム
}

#[account]
pub struct Vault {
    pub authority: Pubkey, // Vaultの権限
}
