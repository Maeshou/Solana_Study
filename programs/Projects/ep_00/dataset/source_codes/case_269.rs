// Secure Case 269: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID269XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_269 {
    pub fn update_license(ctx: Context<Accounts269>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error269::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let increment = bal.checked_add(amount).unwrap();
        **info.try_borrow_mut_lamports()? = increment;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts269<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct269>,
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
pub struct DataStruct269 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error269 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
