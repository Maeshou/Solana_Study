// Secure Case 280: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID280XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_280 {
    pub fn perform_data_compression(ctx: Context<Accounts280>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error280::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        **info.try_borrow_mut_lamports()? = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts280<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct280>,
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
pub struct DataStruct280 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error280 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
