#[program]
pub mod fixed_reset_owner {
    use super::*;

    pub fn reset_owner(ctx: Context<ResetOwner>, new_owner: Pubkey) -> Result<()> {
        let account = &mut ctx.accounts.ownership_account;
        require_keys_eq!(account.owner, ctx.accounts.caller.key(), Unauthorized);
        account.owner = new_owner;
        account.verified = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResetOwner<'info> {
    #[account(init_if_needed, payer = caller, space = 8 + 32 + 1)]
    pub ownership_account: Account<'info, Ownership>,
    #[account(mut)]
    pub caller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Ownership {
    pub owner: Pubkey,
    pub verified: bool,
}

#[error_code]
pub enum ErrorCode {
    Unauthorized,
}