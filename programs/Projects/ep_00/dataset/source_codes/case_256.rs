// Secure Case 256: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID256XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_256 {
    pub fn commit_charity(ctx: Context<Accounts256>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error256::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        **info.try_borrow_mut_lamports()? = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts256<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct256>,
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
pub struct DataStruct256 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error256 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
