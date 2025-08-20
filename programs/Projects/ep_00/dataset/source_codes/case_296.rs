// Secure Case 296: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID296XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_296 {
    pub fn commit_access(ctx: Context<Accounts296>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error296::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        **info.try_borrow_mut_lamports()? = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts296<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct296>,
    #[account(mut)]
    pub account: AccountInfo<'info>,
    #[account(mut)]
    pub src_acc: AccountInfo<'info>,
    #[account(mut)]
    pub dst_acc: AccountInfo<'info>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

#[account]
pub struct DataStruct296 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error296 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
