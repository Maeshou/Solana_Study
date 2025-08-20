// Secure Case 251: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID251XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_251 {
    pub fn process_burn(ctx: Context<Accounts251>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error251::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let doubled = bal * 2;
        **info.try_borrow_mut_lamports()? = doubled;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts251<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct251>,
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
pub struct DataStruct251 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error251 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
