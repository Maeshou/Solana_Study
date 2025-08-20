#[program]
pub mod fixed_reset_data {
    use super::*;

    pub fn reset_data(ctx: Context<ResetData>) -> Result<()> {
        let data = &mut ctx.accounts.data_account;
        require_keys_eq!(data.owner, ctx.accounts.user.key(), Unauthorized);
        data.value = 0;
        data.flag = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetData<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 16 + 32)]
    pub data_account: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub value: u64,
    pub flag: bool,
    pub owner: Pubkey, // ← 所有者フィールド追加
}

#[error_code]
pub enum ErrorCode {
    Unauthorized,
}