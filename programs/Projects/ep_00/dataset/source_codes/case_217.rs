// Secure Case 217: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID217XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_217 {
    pub fn dispatch_reward(ctx: Context<Accounts217>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error217::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        let bal = **info.lamports.borrow();
        let increment = bal.checked_add(amount).unwrap();
        **info.try_borrow_mut_lamports()? = increment;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts217<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct217>,
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
pub struct DataStruct217 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error217 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
