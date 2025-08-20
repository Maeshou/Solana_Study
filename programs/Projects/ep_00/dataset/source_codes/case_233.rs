// Secure Case 233: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID233XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_233 {
    pub fn operate_burn(ctx: Context<Accounts233>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error233::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let increment = bal.checked_add(amount).unwrap();
        **info.try_borrow_mut_lamports()? = increment;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts233<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct233>,
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
pub struct DataStruct233 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error233 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
