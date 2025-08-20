#[program]
pub mod fixed_reset_balance {
    use super::*;

    pub fn reset_balance(ctx: Context<ResetBalance>, new_balance: u64) -> Result<()> {
        let account = &mut ctx.accounts.user_account;
        require_keys_eq!(account.owner, ctx.accounts.payer.key(), Unauthorized);
        account.balance = new_balance;
        account.status = "reset".to_string();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetBalance<'info> {
    #[account(init_if_needed, payer = payer, space = 8 + 64)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
    pub status: String,
    pub owner: Pubkey, // ← 所有者を追加して検証可能に
}

#[error_code]
pub enum ErrorCode {
    Unauthorized,
}