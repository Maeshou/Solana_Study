// Secure Case 231: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID231XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_231 {
    pub fn process_vote(ctx: Context<Accounts231>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error231::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let doubled = bal * 2;
        **info.try_borrow_mut_lamports()? = doubled;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts231<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct231>,
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
pub struct DataStruct231 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error231 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
