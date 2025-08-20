// Secure Case 285: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID285XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_285 {
    pub fn apply_snapshot(ctx: Context<Accounts285>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error285::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let increment = bal.checked_add(amount).unwrap();
        **info.try_borrow_mut_lamports()? = increment;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts285<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct285>,
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
pub struct DataStruct285 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error285 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
