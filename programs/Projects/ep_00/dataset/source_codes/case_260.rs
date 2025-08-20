// Secure Case 260: 多様化された安全コード
use anchor_lang::prelude::*;
declare_id!("SecID260XXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod case_260 {
    pub fn perform_micropay(ctx: Context<Accounts260>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault_data;
        require_keys_eq(vault.owner, ctx.accounts.owner.key(), Error260::InvalidOwner);
        
        let info = ctx.accounts.account.to_account_info();
        **info.try_borrow_mut_lamports()? = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Accounts260<'info> {
    #[account(mut, has_one = owner)]
    pub vault_data: Account<'info, DataStruct260>,
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
pub struct DataStruct260 {
    pub owner: Pubkey,
}

#[error_code]
pub enum Error260 {
    #[msg("Invalid owner")]
    InvalidOwner,
}
