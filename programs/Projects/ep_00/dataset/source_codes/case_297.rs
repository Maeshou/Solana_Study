// Secure Case 297: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID297XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_297 {
    pub fn dispatch_auth(ctx: Context<Accounts297>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error297::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let increment = bal.checked_add(amount).unwrap();
        **info.try_borrow_mut_lamports()? = increment;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts297<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct297>,
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
pub struct DataStruct297 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error297 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
