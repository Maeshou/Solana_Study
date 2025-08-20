// Secure Case 255: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID255XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_255 {
    pub fn apply_royalty(ctx: Context<Accounts255>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error255::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let doubled = bal * 2;
        **info.try_borrow_mut_lamports()? = doubled;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts255<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct255>,
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
pub struct DataStruct255 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error255 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
